#[derive(Clone, Copy, PartialEq)]
pub struct Keyword(pub &'static str);

pub static DOCKER_COMPOSE_KEYWORDS: &[Keyword] = &[
    Keyword("version"), Keyword("name"), Keyword("services"), 
    Keyword("volumes"), Keyword("configs"), Keyword("networks"),
    Keyword("secrets"), Keyword("include"), Keyword("image"), Keyword("build"),
    Keyword("context"), Keyword("dockerfile"), Keyword("args"),
    Keyword("command"), Keyword("entrypoint"), Keyword("ports"), Keyword("expose"),
    Keyword("environment"), Keyword("env_file"), Keyword("depends_on"),
    Keyword("restart"), Keyword("deploy"), Keyword("labels"), Keyword("healthcheck"),
    Keyword("working_dir"), Keyword("user"), Keyword("privileged"), Keyword("cap_add"),
    Keyword("cap_drop"), Keyword("devices"), Keyword("dns"), Keyword("dns_search"),
    Keyword("tmpfs"), Keyword("ulimits"), Keyword("stdin_open"), Keyword("tty"),
    Keyword("profiles"), Keyword("scale"), Keyword("init"), Keyword("pid"), Keyword("ipc"),
    Keyword("isolation"), Keyword("cpu_count"), Keyword("cpu_percent"), Keyword("cpu_shares"),
    Keyword("cpu_quota"), Keyword("cpus"), Keyword("mem_limit"), Keyword("mem_reservation"),
    Keyword("memswap_limit"), Keyword("shm_size"), Keyword("oom_score_adj"), Keyword("target"), 
    Keyword("cache_from"), Keyword("cache_to"), Keyword("extra_hosts"), Keyword("logging"),
    Keyword("security_opt"), Keyword("stop_grace_period"), Keyword("stop_signal"),
    Keyword("sysctls"), Keyword("platform"), Keyword("driver"), Keyword("driver_opts"),
    Keyword("external"), Keyword("ipam"), Keyword("attachable"), Keyword("internal"), 
    Keyword("ipv6"), Keyword("file"), Keyword("content"),
];

pub static RUST_KEYWORDS: &[Keyword] = &[
    Keyword("fn"), Keyword("let"), Keyword("mut"), Keyword("match"), Keyword("if"),
    Keyword("else"), Keyword("loop"), Keyword("while"), Keyword("for"), Keyword("in"),
    Keyword("impl"), Keyword("struct"), Keyword("enum"), Keyword("trait"), Keyword("pub"),
    Keyword("use"), Keyword("mod"), Keyword("where"), Keyword("type"), Keyword("move"),
    Keyword("async"), Keyword("await"), Keyword("return"), Keyword("break"),
    Keyword("continue"), Keyword("const"), Keyword("static"), Keyword("unsafe"),
    Keyword("dyn"), Keyword("as"), Keyword("ref"), Keyword("true"), Keyword("false"),
    Keyword("self"), Keyword("Self"), Keyword("crate"), Keyword("super"),
];

pub static SQL_KEYWORDS: &[Keyword] = &[
    Keyword("SELECT"), Keyword("FROM"), Keyword("WHERE"), Keyword("INSERT"),
    Keyword("INTO"), Keyword("VALUES"), Keyword("UPDATE"), Keyword("SET"),
    Keyword("DELETE"), Keyword("CREATE"), Keyword("TABLE"), Keyword("DROP"),
    Keyword("ALTER"), Keyword("ADD"), Keyword("JOIN"), Keyword("INNER"),
    Keyword("LEFT"), Keyword("RIGHT"), Keyword("FULL"), Keyword("ON"),
    Keyword("GROUP"), Keyword("BY"), Keyword("ORDER"), Keyword("ASC"),
    Keyword("DESC"), Keyword("DISTINCT"), Keyword("LIMIT"), Keyword("OFFSET"),
    Keyword("UNION"), Keyword("ALL"), Keyword("AND"), Keyword("OR"),
    Keyword("NOT"), Keyword("NULL"), Keyword("IS"), Keyword("IN"),
    Keyword("LIKE"), Keyword("BETWEEN"), Keyword("EXISTS"), Keyword("CASE"),
    Keyword("WHEN"), Keyword("THEN"), Keyword("ELSE"), Keyword("END"),
    Keyword("PRIMARY"), Keyword("KEY"), Keyword("FOREIGN"), Keyword("REFERENCES"),
    Keyword("INDEX"), Keyword("VIEW"), Keyword("AS"), Keyword("HAVING"),
];

pub static ASM_X86_KEYWORDS: &[Keyword] = &[
    Keyword("mov"), Keyword("add"), Keyword("adc"), Keyword("sub"), Keyword("sbb"),
    Keyword("inc"), Keyword("dec"), Keyword("mul"), Keyword("imul"), Keyword("div"),
    Keyword("idiv"), Keyword("and"), Keyword("or"), Keyword("xor"), Keyword("not"),
    Keyword("neg"), Keyword("shl"), Keyword("shr"), Keyword("sal"), Keyword("sar"),
    Keyword("rol"), Keyword("ror"), Keyword("rcl"), Keyword("rcr"), Keyword("cmp"),
    Keyword("test"), Keyword("push"), Keyword("pop"), Keyword("pushf"), Keyword("popf"),
    Keyword("pusha"), Keyword("popa"), Keyword("lea"), Keyword("xchg"),
    Keyword("movsx"), Keyword("movzx"), Keyword("bswap"), Keyword("jmp"), Keyword("je"),
    Keyword("jne"), Keyword("jg"), Keyword("jl"), Keyword("jge"), Keyword("jle"),
    Keyword("ja"), Keyword("jb"), Keyword("jae"), Keyword("jbe"), Keyword("call"),
    Keyword("ret"), Keyword("int"), Keyword("iret"), Keyword("loop"), Keyword("loope"),
    Keyword("loopne"), Keyword("rep"), Keyword("repe"), Keyword("repne"),
    Keyword("stos"), Keyword("lods"), Keyword("movs"), Keyword("cmps"), Keyword("scas"),
    Keyword("cli"), Keyword("sti"), Keyword("hlt"), Keyword("nop"),
    Keyword("syscall"), Keyword("sysret"), Keyword("movaps"), Keyword("addps"),
    Keyword("mulps"), Keyword("movsd"), Keyword("addsd"), Keyword("mulsd"), 
    Keyword("vmovdqa64"), Keyword("vpcmpneqq"), Keyword("kmovd"), Keyword("vzeroupper"),
    Keyword("vmovdqu"), Keyword("vmovdqa"), Keyword("vmovmskpd"), Keyword("vpmovmskb"),
    Keyword("vmovdqu8"), Keyword("vpcmpneqb"), Keyword("kmovq"), Keyword("vpcmpeqq"),
    Keyword("vpcmpeqb"), Keyword("jz"), Keyword("jnz")
];

pub static ASM_8051_KEYWORDS: &[Keyword] = &[
    Keyword("NOP"), Keyword("AJMP"), Keyword("LJMP"), Keyword("SJMP"), Keyword("JMP"),
    Keyword("ACALL"), Keyword("LCALL"), Keyword("RET"), Keyword("RETI"), Keyword("RR"),
    Keyword("RL"), Keyword("RRC"), Keyword("RLC"), Keyword("INC"), Keyword("DEC"),
    Keyword("ADD"), Keyword("ADDC"), Keyword("SUBB"), Keyword("DA"), Keyword("ANL"),
    Keyword("ORL"), Keyword("XRL"), Keyword("CLR"), Keyword("CPL"), Keyword("MOV"),
    Keyword("MOVC"), Keyword("MOVX"), Keyword("XCH"), Keyword("PUSH"), Keyword("POP"),
    Keyword("SWAP"), Keyword("MUL"), Keyword("DIV"), Keyword("JB"), Keyword("JNB"),
    Keyword("JC"), Keyword("JNC"), Keyword("JZ"), Keyword("JNZ"), Keyword("JBC"),
    Keyword("DJNZ"), Keyword("CJNE"),
];

pub static PYTHON_KEYWORDS: &[Keyword] = &[
    Keyword("def"), Keyword("class"), Keyword("import"), Keyword("from"), Keyword("for"),
    Keyword("while"), Keyword("if"), Keyword("elif"), Keyword("else"), Keyword("return"),
    Keyword("True"), Keyword("False"), Keyword("None"), Keyword("with"), Keyword("as"),
    Keyword("try"), Keyword("except"), Keyword("finally"), Keyword("in"), Keyword("is"),
    Keyword("not"),
];

pub static JS_KEYWORDS: &[Keyword] = &[
    Keyword("function"), Keyword("const"), Keyword("let"), Keyword("var"), Keyword("if"),
    Keyword("else"), Keyword("for"), Keyword("while"), Keyword("return"), Keyword("class"),
    Keyword("import"), Keyword("from"), Keyword("export"), Keyword("true"), Keyword("false"),
    Keyword("null"), Keyword("undefined"), Keyword("new"), Keyword("this"),
];

pub static KEYWORDS_C: &[Keyword] = &[
    Keyword("int"), Keyword("char"), Keyword("float"), Keyword("double"), Keyword("void"),
    Keyword("return"), Keyword("if"), Keyword("else"), Keyword("for"), Keyword("while"),
    Keyword("struct"), Keyword("typedef"), Keyword("enum"), Keyword("const"),
    Keyword("static"), Keyword("unsigned"), Keyword("signed"), Keyword("long"),
    Keyword("short"), Keyword("include"), Keyword("define"), Keyword("ifdef"), Keyword("endif"),
    Keyword("pragma"), Keyword("error"), Keyword("do"), Keyword("ifndef"),
    Keyword("size_t"), Keyword("ssize_t"), Keyword("undef"), 
    Keyword("defined"), Keyword("ifndef"), Keyword("elif"), Keyword("goto")
];

pub static SHELL_KEYWORDS: &[Keyword] = &[
    Keyword("if"), Keyword("then"), Keyword("else"), Keyword("fi"), Keyword("for"),
    Keyword("in"), Keyword("do"), Keyword("done"), Keyword("case"), Keyword("esac"),
    Keyword("while"), Keyword("function"), Keyword("elif"),
];

pub static DOCKER_KEYWORDS: &[Keyword] = &[
    Keyword("FROM"), Keyword("RUN"), Keyword("CMD"), Keyword("ENTRYPOINT"), Keyword("COPY"),
    Keyword("ADD"), Keyword("ENV"), Keyword("ARG"), Keyword("WORKDIR"), Keyword("EXPOSE"),
    Keyword("USER"), Keyword("VOLUME"), Keyword("LABEL"), Keyword("ONBUILD"),
];

pub static JAVA_KEYWORDS: &[Keyword] = &[
    Keyword("abstract"), Keyword("assert"), Keyword("boolean"), Keyword("break"), Keyword("byte"),
    Keyword("case"), Keyword("catch"), Keyword("char"), Keyword("class"), Keyword("const"),
    Keyword("continue"), Keyword("default"), Keyword("do"), Keyword("double"), Keyword("else"),
    Keyword("enum"), Keyword("extends"), Keyword("final"), Keyword("finally"), Keyword("float"),
    Keyword("for"), Keyword("goto"), Keyword("if"), Keyword("implements"), Keyword("import"),
    Keyword("instanceof"), Keyword("int"), Keyword("interface"), Keyword("long"), Keyword("native"),
    Keyword("new"), Keyword("package"), Keyword("private"), Keyword("protected"), Keyword("public"),
    Keyword("return"), Keyword("short"), Keyword("static"), Keyword("strictfp"), Keyword("super"),
    Keyword("switch"), Keyword("synchronized"), Keyword("this"), Keyword("throw"), Keyword("throws"),
    Keyword("transient"), Keyword("try"), Keyword("void"), Keyword("volatile"), Keyword("while"),
    Keyword("true"), Keyword("false"), Keyword("null"),
    Keyword("var"), Keyword("yield"), Keyword("record"), Keyword("sealed"), Keyword("permits"),
    Keyword("non-sealed"), Keyword("when"),
];

