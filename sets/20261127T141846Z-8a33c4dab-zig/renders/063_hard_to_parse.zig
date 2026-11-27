const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

fn sh2FileTest(flag: []const u8, p: []const u8) bool {
    const st = std.fs.cwd().statFile(p) catch return false;
    if (std.mem.eql(u8, flag, "-f")) return st.kind == .file;
    if (std.mem.eql(u8, flag, "-d")) return st.kind == .directory;
    if (std.mem.eql(u8, flag, "-e")) return true;
    if (std.mem.eql(u8, flag, "-s")) return st.size != 0;
    return false;
}

fn sh2WriteFile(p: []const u8, data: []const u8, append: bool) void {
    const f = std.fs.cwd().createFile(p, .{ .truncate = !append }) catch return;
    defer f.close();
    f.writeAll(data) catch {};
}

fn matchGlob(s: []const u8, pat: []const u8) bool {
    var si: usize = 0;
    var pi: usize = 0;
    var ssi: usize = 0;
    var spi: usize = 0;
    var star = false;
    while (si < s.len) {
        if (pi < pat.len) {
            const pc = pat[pi];
            if (pc == '*') {
                star = true;
                ssi = si;
                spi = pi;
                pi += 1;
                continue;
            }
            if (pc == '?' or s[si] == pc) {
                si += 1;
                pi += 1;
                continue;
            }
        }
        if (star) { si = ssi + 1; ssi = si; pi = spi + 1; } else return false;
    }
    while (pi < pat.len and pat[pi] == '*') { pi += 1; }
    return pi == pat.len;
}

fn sh2Run(argv: []const []const u8) []const u8 {
    const io = sh2_io orelse return "";
    const result = std.process.run(std.heap.page_allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1 << 20),
    }) catch return "";
    return std.mem.trimEnd(u8, result.stdout, "\n");
}

fn sh2ArithStub() i64 {
    sh2TODO("arith");
    return 0;
}

fn sh2ArrayIndex() []const u8 {
    sh2TODO("arrayIndex");
    return "";
}

fn sh2ArrayItems() []const u8 {
    sh2TODO("arrayItems");
    return "";
}

fn sh2ArrayLen() []const u8 {
    sh2TODO("arrayLen");
    return "";
}

fn sh2Block() []const u8 {
    sh2TODO("block");
    return "";
}

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2GetVar() []const u8 {
    sh2TODO("getVar");
    return "";
}

fn sh2Join() []const u8 {
    sh2TODO("join");
    return "";
}

fn sh2ListVar() []const u8 {
    sh2TODO("listVar");
    return "";
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}

fn sh2StrHasPrefix() []const u8 {
    sh2TODO("strHasPrefix");
    return "";
}

fn sh2TestStub() bool {
    sh2TODO("test");
    return false;
}


fn complex_function(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    _ = __args;
    var i: []const u8 = "";
    // TODO(unsupported): listVar → sh2.listVar
    // TODO(unsupported): listVar → sh2.listVar
    _ = sh2Run(&[_][]const u8{"local", "-a", [_] []const u8{sh2ListVar()}});
    _ = sh2Run(&[_][]const u8{"local", "-A", [_] []const u8{}});
    i = "0";
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    while (sh2Truthy(sh2Exec())) {
        // TODO(unsupported): param → sh2.param
        // TODO(unsupported): param → sh2.param
        __sh2_casesubj_2 = std.fmt.allocPrint(std.heap.page_allocator, "{s}", .{sh2Param()}) catch "";
        // TODO(unsupported): strHasPrefix → sh2.strHasPrefix
        // TODO(unsupported): strHasPrefix → sh2.strHasPrefix
        if (sh2Truthy(sh2StrHasPrefix())) {
            // TODO(unsupported): param → sh2.param
            // TODO(unsupported): param → sh2.param
            _ = sh2Run(&[_][]const u8{"local", "key=", sh2Param()});
            // TODO(unsupported): param → sh2.param
            // TODO(unsupported): param → sh2.param
            _ = sh2Run(&[_][]const u8{"local", "value=", sh2Param()});
            options___key__ = sh2Env("value");
            i = sh2IntStr(sh2ArithStub());
        // TODO(unsupported): strHasPrefix → sh2.strHasPrefix
        // TODO(unsupported): strHasPrefix → sh2.strHasPrefix
        } else if (sh2Truthy(sh2StrHasPrefix())) {
            // TODO(unsupported): param → sh2.param
            // TODO(unsupported): param → sh2.param
            _ = sh2Run(&[_][]const u8{"local", "flags=", sh2Param()});
            j = "0";
            // TODO(unsupported): exec → sh2.exec
            // TODO(unsupported): exec → sh2.exec
            while (sh2Truthy(sh2Exec())) {
                options____flags_j_1___ = "true";
                j = sh2IntStr(sh2ArithStub());
            }
            i = sh2IntStr(sh2ArithStub());
        } else {
            break;
        }
    }
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    return std.fmt.allocPrint(std.heap.page_allocator, "Processed {s} options", .{sh2Join()}) catch "";
}
pub fn main(init: std.process.Init) !void {

    var __ps_tmp0: []const u8 = "";
    var __ps_tmp1: []const u8 = "";
    var __ps_tmp2: []const u8 = "";
    var __sh2_casesubj_0: []const u8 = "";
    const __sh2_casesubj_2: []const u8 = "";
    const a: []const u8 = "";
    const b: []const u8 = "";
    const c: []const u8 = "";
    var counter: i64 = 0;
    const d: []const u8 = "";
    const e: []const u8 = "";
    const f: []const u8 = "";
    const g: []const u8 = "";
    const h: []const u8 = "";
    var j: []const u8 = "";
    const k: []const u8 = "";
    const l: []const u8 = "";
    const m: []const u8 = "";
    var matrix_0_0_: []const u8 = "";
    var matrix_1_1_: []const u8 = "";
    var matrix_2_2_: []const u8 = "";
    var matrix_3_3_: []const u8 = "";
    var n: []const u8 = "";
    const options___key__: []const u8 = "";
    const options____flags_j_1___: []const u8 = "";
    var output: []const u8 = "";
    var result: []const u8 = "";
    var result_i_: []const u8 = "";
    const x: []const u8 = "";
    const y: []const u8 = "";
    const z: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    result = sh2IntStr((((@divTrunc(((sh2ToInt(a) + sh2ToInt(b)) * (sh2ToInt(c) - sh2ToInt(d))), @rem(sh2ToInt(e), sh2ToInt(f))) + std.math.pow(i64, sh2ToInt(g), sh2ToInt(h))) - (sh2ToInt(i) << sh2ToInt(j))) | ((sh2ToInt(k) & sh2ToInt(l)) ^ (sh2ToInt(m) | sh2ToInt(n))))); // line 7
    _ = sh2Run(&[_][]const u8{"declare", "-A", "matrix"}); // line 10
    matrix_0_0_ = sh2IntStr(((sh2ToInt(x) + sh2ToInt(y)) * sh2ToInt(z))); // line 11
    // TODO(unsupported): arrayIndex → sh2.arrayIndex // line 12
    // TODO(unsupported): arrayIndex → sh2.arrayIndex
    matrix_1_1_ = sh2ArrayIndex();
    // TODO(unsupported): arrayItems → sh2.arrayItems // line 13
    // TODO(unsupported): arrayItems → sh2.arrayItems
    matrix_2_2_ = sh2ArrayItems();
    // TODO(unsupported): arrayLen → sh2.arrayLen // line 14
    // TODO(unsupported): arrayLen → sh2.arrayLen
    matrix_3_3_ = sh2ArrayLen();
    output = sh2Capture();
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 21
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 22
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    _ = sh2Run(&[_][]const u8{"sleep", "1"}); // line 33
    try stdout.print("{s}\n", .{"Starting"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"sleep", "2"}); // line 34
    try stdout.print("{s}\n", .{"Processing"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"wait"}); // line 35
    try stdout.print("{s}\n", .{"All done"});
    try stdout.flush();
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    if (((std.mem.eql(u8, sh2GetVar(), sh2GetVar()) and sh2Truthy(sh2Exec())) and sh2FileTest("-f", sh2GetVar()))) {
        // TODO(unsupported): exec → sh2.exec
        // TODO(unsupported): exec → sh2.exec
        if ((std.mem.eql(u8, sh2GetVar(), "~\"value") or sh2Truthy(sh2Exec()))) {
            if (sh2Test()) {
                try stdout.print("{s}\n", .{"Deeply nested condition met"});
    try stdout.flush();
            }
        }
    }
    if (matchGlob(sh2Capture(), "*[0-9]*")) {
        // TODO(unsupported): param → sh2.param
        // TODO(unsupported): param → sh2.param
        __sh2_casesubj_0 = std.fmt.allocPrint(std.heap.page_allocator, "{s}", .{sh2Param()}) catch "";
        if (std.mem.indexOf(u8, __sh2_casesubj_0, "pattern") != null) {
            try stdout.print("{s}\n", .{"Double nested pattern"});
    try stdout.flush();
        } else {
            try stdout.print("{s}\n", .{"Single nested pattern"});
    try stdout.flush();
        }
    } else {
        try stdout.print("{s}\n", .{"No numbers"});
    try stdout.flush();
    }
    i = sh2IntStr(sh2ArithStub()); // line 97
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    while (sh2Truthy(sh2Exec())) {
        j = sh2IntStr(sh2ArithStub());
        // TODO(unsupported): exec → sh2.exec
        // TODO(unsupported): exec → sh2.exec
        while (sh2Truthy(sh2Exec())) {
            // TODO(unsupported): exec → sh2.exec
            // TODO(unsupported): exec → sh2.exec
            if (sh2Truthy(sh2Exec())) {
                result_i_ = sh2IntStr(sh2ArithStub());
            }
            j = sh2IntStr(sh2ArithStub());
        }
        i = sh2IntStr(sh2ArithStub());
    }
    __ps_tmp0 = sh2Capture();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    sh2WriteFile(__ps_tmp0, "", false);
    // TODO(unsupported): block → sh2.block
    // TODO(unsupported): block → sh2.block
    while (sh2Truthy(sh2Block())) {
        if (std.mem.eql(u8, sh2GetVar(), "~^[[:space:]]*#")) {
            continue;
        }
        if (matchGlob(sh2Env("line"), "*\\$\\(*\\)*")) {
            try stdout.print("{s}{s}\n", .{"Contains command substitution: ", sh2Env("line")});
    try stdout.flush();
        } else if (matchGlob(sh2Env("line"), "*\\$\\{[^}]*\\}*")) {
            try stdout.print("{s}{s}\n", .{"Contains parameter expansion: ", sh2Env("line")});
    try stdout.flush();
        } else if (matchGlob(sh2Env("line"), "*\\$\\(\\(*\\)\\)*")) {
            try stdout.print("{s}{s}\n", .{"Contains arithmetic expansion: ", sh2Env("line")});
    try stdout.flush();
        }
        counter += 1;
    }
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = sh2Exec();
    _ = sh2Run(&[_][]const u8{"eval", "result=$(( ${var:-0} + ${array[${index:-0}]:-0} ))"});
    // TODO(unsupported): pipeline → sh2.pipeline // line 130
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    __ps_tmp1 = sh2Capture();
    _ = sh2Run(&[_][]const u8{"sort", "file1.txt"});
    sh2WriteFile(__ps_tmp1, "", false);
    __ps_tmp2 = sh2Capture();
    _ = sh2Run(&[_][]const u8{"sort", "file2.txt"});
    sh2WriteFile(__ps_tmp2, "", false);
    _ = sh2Run(&[_][]const u8{"diff", __ps_tmp1, __ps_tmp2});
    sh2WriteFile("comparison.txt", "", false);
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = sh2Exec();
    if ((sh2Test() and sh2Test())) {
        try stdout.print("{s}\n", .{"Complex test passed"});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{"a1x a1y a1z a2x a2y a2z a3x a3y a3z b1x b1y b1z b2x b2y b2z b3x b3y b3z c1x c1y c1z c2x c2y c2z c3x c3y c3z"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"tr", "[:upper:]", "[:lower:]"}); // line 161
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{complex_function(init, &[_][]const u8{ "--long-option=value with spaces", "--array-option", "item1", "item2", "item3", "--flag", "positional argument", sh2Param(), "computed" })});
    try stdout.flush();
    // TODO(unsupported): pipeline → sh2.pipeline // line 173
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    _ = &matrix_0_0_;
    _ = &matrix_1_1_;
    _ = &matrix_2_2_;
    _ = &matrix_3_3_;
    _ = &output;
    _ = &result_i_;
}
// 60 construct(s) lowered to TODO markers