#[cfg(target_os = "linux")]
fn main() {
    example::main();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("This example is currently not supported for OSes other than Linux");
}

#[cfg(target_os = "linux")]
mod example {
    use std::net;
    use std::num::NonZeroU32;
    use std::path::PathBuf;
    use std::process::Command;

    use dap::msg::dap_type::breakpoint::{self, Breakpoint};
    use dap::msg::dap_type::stack_frame::StackFrame;
    use dap::msg::event::stopped::Reason;
    use dap::msg::event::StoppedEvent;
    use dap::msg::request::{ContinueResponse, StackTraceResponse};
    use dap::{
        codec::DapCodec,
        msg::{
            dap_type::{thread::Thread, Capabilities},
            event::{Event, InitializedEvent},
            request::{
                AcknowledgementResponse, DisconnectResponse, InitializeResponse, PauseResponse,
                Request, Response, ResponseType, SetBreakpointsResponse,
                SetExceptionBreakpointsResponse, ThreadsResponse,
            },
            MsgType,
        },
    };
    use headcrab::{
        symbol::{DisassemblySource, RelocatedDwarf},
        target::{LinuxTarget, Registers, UnixTarget},
        CrabResult,
    };

    #[cfg(target_os = "linux")]
    #[derive(Default)]
    struct Context {
        remote: Option<LinuxTarget>,
        debuginfo: Option<RelocatedDwarf>,
        disassembler: DisassemblySource,
    }

    /// The subcommands that are acceptable for backtrace.
    enum BacktraceType {
        // uses the frame_pointer_unwinder.
        FramePtr,

        // uses naive_unwinder.
        Naive,
    }

    #[derive(Debug)]
    struct BacktraceTypeError(String);

    impl std::fmt::Display for BacktraceTypeError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Unrecognized backtrace type {}. Supported ones are 'fp' and 'naive'. Please consider using one of them.", self.0.trim())
        }
    }
    impl std::error::Error for BacktraceTypeError {}

    impl BacktraceType {
        #[inline]
        fn from_str(value: &str) -> Result<Self, BacktraceTypeError> {
            match value {
                "fp" | "" => Ok(BacktraceType::FramePtr),
                "naive" => Ok(BacktraceType::Naive),
                _ => Err(BacktraceTypeError(value.to_owned()).into()),
            }
        }
    }

    impl Default for BacktraceType {
        fn default() -> Self {
            BacktraceType::FramePtr
        }
    }

    /*
        impl HighlightAndComplete for BacktraceType {
            type Error = BacktraceTypeError;
            fn from_str(line: &str) -> Result<Self, Self::Error> {
                BacktraceType::from_str(line.trim())
            }

            fn highlight<'l>(line: &'l str) -> Cow<'l, str> {
                line.into()
            }

            fn complete(
                line: &str,
                pos: usize,
                ctx: &rustyline::Context<'_>,
            ) -> rustyline::Result<(usize, Vec<Pair>)> {
                let pos_first_non_whitespace = line
                    .chars()
                    .position(|c| !c.is_ascii_whitespace())
                    .unwrap_or(0);

                let candidates = ["fp", "naive"]
                    .iter()
                    .filter(|&&cmd| cmd.starts_with(&line.trim_start().to_lowercase()))
                    .map(|cmd| Pair {
                        display: String::from(*cmd),
                        replacement: String::from(*cmd) + " ",
                    })
                    .collect::<Vec<_>>();

                let _ = (line, pos, ctx);
                return Ok((pos_first_non_whitespace, candidates));
            }
        }
    */
    impl Context {
        fn remote(&self) -> CrabResult<&LinuxTarget> {
            if let Some(remote) = &self.remote {
                Ok(remote)
            } else {
                Err("No running process".to_string().into())
            }
        }

        fn mut_remote(&mut self) -> CrabResult<&mut LinuxTarget> {
            if let Some(remote) = &mut self.remote {
                Ok(remote)
            } else {
                Err("No running process".to_string().into())
            }
        }

        fn set_remote(&mut self, remote: LinuxTarget) {
            // FIXME kill/detach old remote
            self.remote = Some(remote);
            self.debuginfo = None;
        }

        fn load_debuginfo_if_necessary(&mut self) -> CrabResult<()> {
            // FIXME only reload debuginfo when necessary (memory map changed)
            let memory_maps = self.remote()?.memory_maps()?;
            self.debuginfo = Some(RelocatedDwarf::from_maps(&memory_maps)?);
            Ok(())
        }

        fn debuginfo(&self) -> &RelocatedDwarf {
            self.debuginfo.as_ref().unwrap()
        }
    }

    pub fn main() {
        let mut context = Context::default();

        let listener = net::TcpListener::bind("127.0.0.1:4711").unwrap();
        let codec = DapCodec::new(listener);

        let cap = Capabilities::default();

        let init_event = Event::Initialized(InitializedEvent {});
        let init_event = MsgType::Event(init_event);

        loop {
            let mut session = codec.accept().unwrap();
            loop {
                let msg = session.recv().unwrap();

                let MsgType::Request(request) = msg.msg_type.clone() else {panic!()};

                let response = match request {
                    Request::Launch(request) => {
                        let path = (request.additional_data.get("program"))
                            .map(serde_json::Value::as_str)
                            .flatten()
                            .unwrap()
                            .to_string();

                        let mut path = PathBuf::from(path);

                        println!("{:?}", path);
                        path.pop();
                        path.pop();

                        println!("{:?}", path);

                        let project_name = path.file_name().unwrap().to_owned();

                        path.push("target/debug/");
                        path.push(project_name);

                        println!("{:?}", path);

                        context.set_remote(match LinuxTarget::launch(Command::new(path)) {
                            Ok((target, status)) => {
                                println!("{:?}", status);
                                target
                            }
                            Err(err) => {
                                println!("Error while launching debuggee: {}", err);
                                std::process::exit(1);
                            }
                        });

                        session.send(init_event.clone()).unwrap();

                        ResponseType::Acknowledgement(AcknowledgementResponse::new(
                            request.command().to_string(),
                        ))
                    }
                    Request::SetExceptionBreakpoints(_) => {
                        let response = SetExceptionBreakpointsResponse { breakpoints: None };
                        ResponseType::SetExceptionBreakpoints(response)
                    }
                    Request::Threads(_) => {
                        let thread_list = context.remote().unwrap().threads().unwrap();

                        let thread_list: Vec<_> = thread_list
                            .iter()
                            .map(|thread| {
                                let id = thread.thread_id() as i64;
                                let name = thread.name().unwrap().unwrap_or(id.to_string());
                                Thread { id, name }
                            })
                            .collect();

                        let response = ThreadsResponse {
                            threads: thread_list,
                        };
                        ResponseType::Threads(response)
                    }
                    Request::Continue(_) => {
                        let response = ContinueResponse {
                            all_threads_continued: Some(true),
                        };

                        context.remote().unwrap().unpause().unwrap();

                        ResponseType::Continue(response)
                    }
                    Request::StackTrace(_) => {
                        let stacktrace = get_stack_trace(&mut context).unwrap();

                        let response = StackTraceResponse {
                            total_frames: Some(stacktrace.len() as u64),
                            stack_frames: stacktrace,
                        };
                        ResponseType::StackTrace(response)
                    }
                    Request::SetBreakpoints(request) => {
                        context.load_debuginfo_if_necessary().unwrap();
                        println!("breakpoint request={:#?}", request);
                        let breakpoint_list = request.breakpoints.clone().unwrap_or_default();

                        for breakpoint in &breakpoint_list {
                            let column = if let Some(column) = breakpoint.column {
                                NonZeroU32::new(column as u32)
                            } else {
                                None
                            };

                            if let Some(addr) = context
                                .debuginfo()
                                .find_location_addr(&addr2line::Location {
                                    file: request.source.path.as_ref().unwrap(),
                                    line: NonZeroU32::new(breakpoint.line as u32).unwrap(),
                                    column,
                                })
                                .unwrap()
                                .values()
                                .next()
                            {
                                context
                                    .remote()
                                    .unwrap()
                                    .set_breakpoint(*addr as usize)
                                    .unwrap();
                            }

                            context.remote().unwrap().unpause().unwrap();
                        }

                        let breakpoint_list = breakpoint_list
                            .iter()
                            .map(|breakpoint| Breakpoint {
                                source: Some(request.source.clone()),
                                column: breakpoint.column,
                                id: None,
                                instruction_reference: None,
                                line: Some(breakpoint.line),
                                end_column: None,
                                offset: None,
                                verified: true,
                                message: None,
                                end_line: None,
                            })
                            .collect();

                        let response = SetBreakpointsResponse {
                            breakpoints: breakpoint_list,
                        };
                        ResponseType::SetBreakpoints(response)
                    }
                    Request::Pause(_) => {
                        let response = PauseResponse {};

                        ResponseType::Pause(response)
                    }
                    Request::Disconnect(_) => {
                        let response = DisconnectResponse {};
                        let response = ResponseType::Disconnect(response);

                        session
                            .send(MsgType::Response(Response {
                                request_seq: msg.seq,
                                response_type: response,
                            }))
                            .unwrap();

                        context.remote().unwrap().kill().unwrap();

                        break;
                    }
                    Request::Initialize(_) => {
                        let response = InitializeResponse {
                            capabilities: Some(cap.clone()),
                        };
                        ResponseType::Initialize(response)
                    }
                    _ => panic!("{:#?}", msg),
                };

                session
                    .send(MsgType::Response(Response {
                        request_seq: msg.seq,
                        response_type: response,
                    }))
                    .unwrap();
            }
        }

        /*loop {
            let command = rl.readline("(headcrab) ");

            match run_command(&mut context, rl.helper().unwrap().color, &command) {
                Ok(()) => {}
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        }*/
    }

    /*
    fn run_command(context: &mut Context, color: bool, command: &str) -> CrabResult<()> {
        if command == "" {
            return Ok(());
        }

        let command = ReplCommand::from_str(command)?;
        match command {
            ReplCommand::Attach(pid) => {
                let pid = nix::unistd::Pid::from_raw(pid.parse()?);
                println!("Attaching to process {}", pid);
                let (remote, status) = LinuxTarget::attach(
                    pid,
                    AttachOptions {
                        kill_on_exit: false,
                    },
                )?;
                println!("{:?}", status);
                // FIXME detach or kill old remote
                context.set_remote(remote);
            }
            ReplCommand::Detach(()) => {
                context.remote()?.detach()?;
                context.remote = None;
            }
            ReplCommand::Breakpoint(location) => set_breakpoint(context, &location)?,
            ReplCommand::Stepi(()) => {
                println!("{:?}", context.remote()?.step()?);
                return print_source_for_top_of_stack_symbol(context, 3);
            }
            ReplCommand::Continue(()) => {
                println!("{:?}", context.remote()?.unpause()?);
                // When we hit the next breakpoint, we also want to display the source code
                // as lldb and gdb does.
                return print_source_for_top_of_stack_symbol(context, 3);
            }
            ReplCommand::Registers(sub_cmd) => match &*sub_cmd {
                "" => Err(format!(
                    "Expected subcommand found nothing. Try `regs read`"
                ))?,
                "read" => println!("{:#016x?}", context.remote()?.read_regs()?),
                _ => Err(format!("Unknown `regs` subcommand `{}`", sub_cmd))?,
            },
            ReplCommand::Backtrace(sub_cmd) => {
                return show_backtrace(context, &sub_cmd);
            }
            ReplCommand::Disassemble(()) => {
                let ip = context.remote()?.read_regs()?.ip();
                let mut code = [0; 64];
                unsafe {
                    context
                        .remote()?
                        .read()
                        .read(&mut code, ip as usize)
                        .apply()?;
                }
                let disassembly = context.disassembler.source_snippet(&code, ip, true)?;
                println!("{}", disassembly);
            }
            ReplCommand::List(()) => {
                return print_source_for_top_of_stack_symbol(context, 3);
            }
            ReplCommand::Locals(()) => {
                return show_locals(context);
            }
            ReplCommand::Exit(()) => unreachable!("Should be handled earlier"),
        }
        Ok(())
    }*/

    /*fn parse_breakpoint(context: &mut Context, location: &str) -> CrabResult<Vec<usize>> {
        context.load_debuginfo_if_necessary()?;
        if let Some(addr) = parse_address(location).or(parse_symbol(&location, context)) {
            Ok(vec![addr])
        } else {
            if let Some(source_location) = parse_source_location(location) {
                Ok(context
                    .debuginfo
                    .as_ref()
                    .unwrap() // this unwrap cannot fail due to the call to load_debuginfo_if_necessary
                    .find_location_addr(&source_location)
                    .unwrap() // todo: this unwrap could fail if the debugee contain invalid debug info.
                    .into_iter()
                    .map(|(_, addr)| addr as usize)
                    .collect())
            } else {
                Err(format!(
                    "Breakpoints must be set on a symbol or at a given address. For example `b main` or `b 0x0000555555559394` or even `b 93824992252820`"
                ))?
            }
        }
    }*/

    fn parse_source_location(location: &str) -> Option<addr2line::Location> {
        use addr2line::Location;

        let mut iter = location.split(":");
        let file = iter.next();
        let line = iter
            .next()
            .map_or(None, parse_address)
            .map_or(None, |num| NonZeroU32::new(num as u32));
        let column = iter
            .next()
            .map_or(None, parse_address)
            .map_or(None, |num| NonZeroU32::new(num as u32));

        if let (Some(file), Some(line)) = (file, line) {
            Some(Location { file, line, column })
        } else {
            None
        }
    }

    fn set_breakpoint(context: &mut Context, addr: usize) -> CrabResult<()> {
        context.mut_remote()?.set_breakpoint(addr)?;
        Ok(())
    }

    fn parse_address(location: &str) -> Option<usize> {
        if let Ok(addr) = usize::from_str_radix(&location, 10) {
            return Some(addr);
        } else {
            if location.starts_with("0x") {
                let raw_num = location.trim_start_matches("0x");
                if let Ok(addr) = usize::from_str_radix(raw_num, 16) {
                    return Some(addr);
                }
            }
        }
        None
    }

    fn parse_symbol(location: &str, context: &mut Context) -> Option<usize> {
        context.debuginfo().get_symbol_address(&location)
    }

    fn get_stack_trace(context: &mut Context) -> CrabResult<Vec<StackFrame>> {
        let call_stack: Vec<_> = get_call_stack(context)?;

        println!("call stack len= {}", call_stack.len());

        let mut output = vec![];
        for func in call_stack {
            println!("stack trace len 0= {}", output.len());
            context.debuginfo().with_addr_frames(
                func,
                |_addr, mut frames: headcrab::symbol::FrameIter| {
                    println!("addr={:X}", func);
                    println!("test");
                    while let Some(frame) = frames.next()? {
                        println!("test2");
                        let name = frame
                            .function
                            .as_ref()
                            .map(|f| Ok(f.demangle()?.into_owned()))
                            .transpose()
                            .map_err(|err: gimli::Error| err)?
                            .unwrap_or_else(|| "<unknown>".to_string());

                        let (line, column) = frame
                            .location
                            .as_ref()
                            .map(|loc| {
                                let line = u32::from(loc.line) as u64;
                                let column = loc.column.map(u32::from).unwrap_or(0) as u64;

                                (line, column)
                            })
                            .unwrap_or_default();

                        output.push(StackFrame {
                            source: None,
                            module_id: None,
                            id: 0,
                            instruction_pointer_reference: None,
                            line,
                            end_column: None,
                            name,
                            column,
                            presentation_hint: None,
                            end_line: None,
                            can_restart: None,
                        });
                    }
                    Ok(())
                },
            )?;

            println!("stack trace len 1= {}", output.len());
        }

        println!("stack trace len= {}", output.len());
        Ok(output)
    }

    fn show_locals(context: &mut Context) -> CrabResult<()> {
        let regs = context.remote()?.main_thread()?.read_regs()?;
        let func = regs.ip() as usize;
        let res = context.debuginfo().with_addr_frames(
            func,
            |func, mut frames: headcrab::symbol::FrameIter| {
                let mut first_frame = true;
                while let Some(frame) = frames.next()? {
                    let name = frame
                        .function
                        .as_ref()
                        .map(|f| Ok(f.demangle()?.into_owned()))
                        .transpose()
                        .map_err(|err: gimli::Error| err)?
                        .unwrap_or_else(|| "<unknown>".to_string());

                    let location = frame
                        .location
                        .as_ref()
                        .map(|loc| format!("{}:{}", loc.file, loc.line))
                        .unwrap_or_default();

                    if first_frame {
                        println!("{:016x} {} {}", func, name, location);
                    } else {
                        println!("                 {} {}", name, location);
                    }

                    let (_dwarf, unit, dw_die_offset) = frame
                        .function_debuginfo()
                        .ok_or_else(|| "No dwarf debuginfo for function".to_owned())?;

                    let mut eval_ctx = EvalContext {
                        frame_base: None,
                        regs: Box::new(regs),
                    };

                    // FIXME handle DW_TAG_inlined_subroutine with DW_AT_frame_base in parent DW_TAG_subprogram
                    if let Some(frame_base) =
                        unit.entry(dw_die_offset)?.attr(gimli::DW_AT_frame_base)?
                    {
                        let frame_base = frame_base.exprloc_value().unwrap();
                        let res = headcrab::symbol::dwarf_utils::evaluate_expression(
                            unit, frame_base, &eval_ctx,
                        )?;
                        assert_eq!(res.len(), 1);
                        assert_eq!(res[0].bit_offset, None);
                        assert_eq!(res[0].size_in_bits, None);
                        match res[0].location {
                            gimli::Location::Register {
                                register: gimli::X86_64::RBP,
                            } => eval_ctx.frame_base = regs.bp(),
                            ref loc => unimplemented!("{:?}", loc), // FIXME
                        }
                    }

                    frame.each_argument(&eval_ctx, func as u64, |local| {
                        show_local("arg", &eval_ctx, local)
                    })?;

                    frame.each_local(&eval_ctx, func as u64, |local| {
                        show_local("    ", &eval_ctx, local)
                    })?;

                    frame.print_debuginfo();

                    first_frame = false;
                }
                Ok(first_frame)
            },
        )?;
        match res {
            Some(true) | None => {
                println!("no locals");
            }
            Some(false) => {}
        }

        Ok(())
    }

    fn get_call_stack(context: &mut Context) -> CrabResult<Vec<usize>> {
        context.load_debuginfo_if_necessary()?;

        let regs = context.remote()?.main_thread()?.read_regs()?;

        let mut stack: [usize; 1024] = [0; 1024];
        unsafe {
            context
                .remote()?
                .read()
                .read(&mut stack, regs.sp() as usize)
                .apply()?;
        }

        let call_stack: Vec<_> = headcrab::symbol::unwind::frame_pointer_unwinder(
            context.debuginfo(),
            &stack[..],
            regs.ip() as usize,
            regs.sp() as usize,
            regs.bp().unwrap() as usize, // TODO: fix `unwrap` for non-x86 platforms,
        )
        .collect();
        Ok(call_stack)
    }

    /*
        /// Gets the call_stack from the context and then tries to display the
        /// source for the top call in the stack. Because the first frame is usually
        /// sse2.rs, we just display the file and line but not the source and we skip
        /// over to the next frame. For the next frame, we will display the source code.
        /// An example view is shown below:
        ///
        /// It marks the line with the berakpoint with a '>' character and shows some lines
        /// of context above and below it.
        ///
        /// ```plain
        /// 0000555555559295 core::core_arch::x86::sse2::_mm_pause /../rustup/toolchains/1.45.2-x86_64-unknown-linux-gnu/../stdarch/crates/core_arch/src/x86/sse2.rs:25
        /// /workspaces/headcrab/tests/testees/hello.rs:7:14
        ///    4 #[inline(never)]
        ///    5 fn breakpoint() {
        ///    6     // This will be patched by the debugger to be a breakpoint
        /// >  7     unsafe { core::arch::x86_64::_mm_pause(); }
        ///    8 }
        ///    9
        ///   10 #[inline(never)]
        /// ```
        fn print_source_for_top_of_stack_symbol(
            context: &mut Context,
            context_lines: usize,
        ) -> CrabResult<()> {
            let call_stack = get_call_stack(context, &BacktraceType::default())?;
            let top_of_stack = call_stack[0];
            context
                .debuginfo()
                .with_addr_frames(top_of_stack, |_addr, mut frames| {
                    while let Some(frame) = frames.next()? {
                        let name = frame
                            .function
                            .as_ref()
                            .map(|f| Ok(f.demangle()?.into_owned()))
                            .transpose()
                            .map_err(|err: gimli::Error| err)?
                            .unwrap_or_else(|| "<unknown>".to_string());

                        let (file, line, column) = frame
                            .location
                            .as_ref()
                            .map(|loc| (loc.file, loc.line, loc.column))
                            .unwrap();

                        let line = u32::from(line);
                        let column = column.map_or(0, u32::from);
                        Snippet::from_file(
                            file,
                            name,
                            line as usize,
                            context_lines as usize,
                            column as usize,
                        )?
                        .highlight();
                        break;
                    }
                    Ok(())
                })?;
            Ok(())
        }
    */
    struct EvalContext {
        frame_base: Option<u64>,
        regs: Box<dyn headcrab::target::Registers>,
    }

    impl headcrab::symbol::dwarf_utils::EvalContext for EvalContext {
        fn frame_base(&self) -> u64 {
            self.frame_base.unwrap()
        }

        fn register(&self, register: gimli::Register, base_type: gimli::ValueType) -> gimli::Value {
            let val = self.regs.reg_for_dwarf(register).unwrap();
            match base_type {
                gimli::ValueType::Generic => gimli::Value::Generic(val),
                gimli::ValueType::U64 => gimli::Value::U64(val),
                _ => unimplemented!(),
            }
        }

        fn memory(
            &self,
            _address: u64,
            _size: u8,
            _address_space: Option<u64>,
            _base_type: gimli::ValueType,
        ) -> gimli::Value {
            todo!()
        }
    }

    fn show_local<'ctx>(
        kind: &str,
        eval_ctx: &EvalContext,
        local: headcrab::symbol::Local<'_, 'ctx>,
    ) -> CrabResult<()> {
        let value = match local.value() {
            value @ headcrab::symbol::LocalValue::Pieces(_)
            | value @ headcrab::symbol::LocalValue::Const(_) => {
                match value.primitive_value(local.type_(), eval_ctx)? {
                    Some(headcrab::symbol::PrimitiveValue::Int { size, signed, data }) => {
                        if signed {
                            (data << (64 - size * 8) >> (64 - size * 8)).to_string()
                        } else {
                            ((data as i64) << (64 - size * 8) >> (64 - size * 8)).to_string()
                        }
                    }
                    Some(headcrab::symbol::PrimitiveValue::Float { is_64, data }) => {
                        if is_64 {
                            f64::from_bits(data).to_string()
                        } else {
                            f32::from_bits(data as u32).to_string()
                        }
                    }
                    None => "<struct>".to_owned(),
                }
            }
            headcrab::symbol::LocalValue::OptimizedOut => "<optimized out>".to_owned(),
            headcrab::symbol::LocalValue::Unknown => "<unknown>".to_owned(),
        };

        println!(
            "{} {} = {}",
            kind,
            local.name()?.unwrap_or("<no name>"),
            value
        );

        Ok(())
    }
}
