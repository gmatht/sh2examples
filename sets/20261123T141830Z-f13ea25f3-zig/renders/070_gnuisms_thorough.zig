const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

fn sh2B2S(b: bool) []const u8 {
    return if (b) "1" else "0";
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

fn sh2Run(argv: []const []const u8) []const u8 {
    const io = sh2_io orelse return "";
    const result = std.process.run(std.heap.page_allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1 << 20),
    }) catch return "";
    return std.mem.trimEnd(u8, result.stdout, "\n");
}

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2Join() []const u8 {
    sh2TODO("join");
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

fn sh2Shopt() []const u8 {
    sh2TODO("shopt");
    return "";
}

fn sh2TestStub() bool {
    sh2TODO("test");
    return false;
}

pub fn main(init: std.process.Init) !void {

    var __cat_line1: []const u8 = "";
    var aa_one_: []const u8 = "";
    var aa_three_: []const u8 = "";
    var aa_two_: []const u8 = "";
    var arr: [1024]i64 = undefined;
    var arr_len: usize = 0;
    var ax: i64 = 0;
    var el: []const u8 = "";
    var k: []const u8 = "";
    var ly: i64 = 0;
    var y: []const u8 = "";
    var z: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    _ = sh2Run(&[_][]const u8{"cd", "/tmp"}); // line 14
    _ = sh2B2S((5 > 3)); // line 17
    try stdout.print("{s}{s}\n", .{"arith:  ", sh2Env("?")});
    try stdout.flush(); // line 17
    _ = sh2B2S(std.mem.eql(u8, "abc", "=\"abc")); // line 18
    try stdout.print("{s}{s}\n", .{"str:    ", sh2Env("?")});
    try stdout.flush(); // line 18
    _ = sh2B2S(sh2FileTest("-f", "/etc/passwd")); // line 19
    try stdout.print("{s}{s}\n", .{"file:   ", sh2Env("?")});
    try stdout.flush(); // line 19
    _ = sh2B2S(sh2Test()); // line 20
    try stdout.print("{s}{s}\n", .{"regex:  ", sh2Env("?")});
    try stdout.flush(); // line 20
    _ = sh2B2S(("" .len == 0)); // line 21
    try stdout.print("{s}{s}\n", .{"empty:  ", sh2Env("?")});
    try stdout.flush(); // line 21
    _ = sh2B2S(("x" .len != 0)); // line 22
    try stdout.print("{s}{s}\n", .{"nonempty: ", sh2Env("?")});
    try stdout.flush(); // line 22
    _ = sh2B2S(("a\"<\"b" .len != 0)); // line 23
    try stdout.print("{s}{s}\n", .{"lt:     ", sh2Env("?")});
    try stdout.flush(); // line 23
    _ = sh2B2S(!std.mem.eql(u8, "a", "b")); // line 24
    try stdout.print("{s}{s}\n", .{"ne:    ", sh2Env("?")});
    try stdout.flush(); // line 24
    _ = sh2B2S(sh2Test()); // line 25
    try stdout.print("{s}{s}\n", .{"notfile: ", sh2Env("?")});
    try stdout.flush(); // line 25
    try stdout.print("{s}{s}\n", .{"up=", "HELLO WORLD"});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"down=", "hello world"});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"firstup=", "HeLLo WoRLD"});
    try stdout.flush();
    y = "one-two-one";
    try stdout.print("{s}{s}\n", .{"gsub=", "1-two-1"});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"sub=", "1-two-one"});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"prefix=", "two-one"});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"suffix=", "one-two"});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 40
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"shortest=", sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 41
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"greedy=", sh2Param()});
    try stdout.flush();
    z = "0123456789";
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"mid=", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): join → sh2.join // line 46
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"tail=", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): join → sh2.join // line 47
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"neg=", sh2Join()});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"len=", "10"});
    try stdout.flush();
    arr_len = 0; // line 52
    arr[arr_len] = sh2ToInt("alpha");
    arr_len += 1;
    arr[arr_len] = sh2ToInt("beta");
    arr_len += 1;
    arr[arr_len] = sh2ToInt("gamma");
    arr_len += 1;
    // TODO(unsupported): param → sh2.param // line 53
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"idx=", sh2Param()});
    try stdout.flush();
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"count=", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): join → sh2.join // line 55
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"all=", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 56
    // TODO(unsupported): param → sh2.param
    {
        const sh2_items0 = [_][]const u8{sh2Param()};
        for (sh2_items0[0..]) |sh2_v1| {
            el = sh2_v1;
            try stdout.print("{s}{s}\n", .{"el=", el});
    try stdout.flush();
        }
    }
    _ = sh2Run(&[_][]const u8{"declare", "-A", "aa"}); // line 60
    aa_one_ = "1"; // line 61
    aa_two_ = "2"; // line 62
    aa_three_ = "3"; // line 63
    // TODO(unsupported): param → sh2.param // line 64
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"aa-one=", sh2Param()});
    try stdout.flush();
    // TODO(unsupported): capture → sh2.capture // line 65
    // TODO(unsupported): capture → sh2.capture
    {
        const sh2_items2 = [_][]const u8{sh2Capture()};
        for (sh2_items2[0..]) |sh2_v3| {
            k = sh2_v3;
            // TODO(unsupported): param → sh2.param
            // TODO(unsupported): param → sh2.param
            try stdout.print("{s}{s}{s}{s}\n", .{"aa[", k, "]=", sh2Param()});
    try stdout.flush();
        }
    }
    _ = sh2Run(&[_][]const u8{"cat"}); // line 68
    try stdout.print("{s}\n", .{"both"});
    try stdout.flush();
    var __f4 = std.Io.Dir.cwd().openFile(init.io, "/tmp/gnu_both.txt", .{}) catch {
        std.process.exit(2);
    };
    defer __f4.close(init.io);
    var __b4: [8192]u8 = undefined;
    var __r4 = __f4.reader(init.io, &__b4);
    while (try __r4.interface.takeDelimiter('\n')) |__ln4| {
        __cat_line1 = __ln4;
        try stdout.print("{s}\n", .{__cat_line1});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{"err-to-stderr"});
    try stdout.flush();
    sh2WriteFile("&2", "", false);
    ax = 8;
    try stdout.print("{s}{d}\n", .{"arith=", ax});
    try stdout.flush(); // line 79
    ly = (ax * 2);
    try stdout.print("{s}{d}\n", .{"let=", ly});
    try stdout.flush(); // line 81
    ax += 1;
    try stdout.print("{s}{d}\n", .{"postfix=", ax});
    try stdout.flush(); // line 83
    try stdout.print("{s}{d}\n", .{"pow=", std.math.pow(i64, 2, 10)});
    try stdout.flush(); // line 84
    std.process.exit(1);
    // TODO(unsupported): param → sh2.param // line 88
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}{s}\n", .{"ps=", sh2Param(), ",", sh2Param()});
    try stdout.flush();
    try stdout.print("{s}", .{"no-nl"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"."});
    try stdout.flush();
    try stdout.print("{s}\n", .{"tab\there"});
    try stdout.flush();
    try stdout.print("{s}{s}", .{"-", "v"});
    try stdout.flush();
    try stdout.print("{s}\n", .{sh2Env("pv")});
    try stdout.flush(); // line 96
    try stdout.print("{s}{s}\n", .{"seq: ", sh2Capture()});
    try stdout.flush(); // line 99
    _ = sh2Run(&[_][]const u8{"ln", "-s", "-f", "/etc/passwd", "/tmp/gnu_link"}); // line 100
    try stdout.print("{s}{s}\n", .{"readlink-f: ", sh2Capture()});
    try stdout.flush(); // line 101
    try stdout.print("{s}{s}\n", .{"readlink-e: ", sh2Capture()});
    try stdout.flush(); // line 102
    _ = sh2Run(&[_][]const u8{"readlink", "-m", "/no/such/path"}); // line 103
    sh2WriteFile("/dev/null", "", false);
    try stdout.print("{s}{s}\n", .{"readlink-m rc=", sh2Env("?")});
    try stdout.flush(); // line 103
    try std.fs.cwd().writeFile(.{ .sub_path = "/tmp/gnu_a.txt", .data = "abc\n" }); // line 105
    try std.fs.cwd().writeFile(.{ .sub_path = "/tmp/gnu_b.txt", .data = "abd\n" }); // line 106
    _ = sh2Run(&[_][]const u8{"cmp", "-b", "/tmp/gnu_a.txt", "/tmp/gnu_b.txt"}); // line 107
    try stdout.print("{s}{s}\n", .{"cmp-b rc=", sh2Env("?")});
    try stdout.flush(); // line 107
    _ = sh2Run(&[_][]const u8{"cmp", "-n", "2", "/tmp/gnu_a.txt", "/tmp/gnu_b.txt"}); // line 108
    try stdout.print("{s}{s}\n", .{"cmp-n rc=", sh2Env("?")});
    try stdout.flush(); // line 108
    _ = sh2Run(&[_][]const u8{"cmp", "-i", "2", "/tmp/gnu_a.txt", "/tmp/gnu_b.txt"}); // line 109
    try stdout.print("{s}{s}\n", .{"cmp-i rc=", sh2Env("?")});
    try stdout.flush(); // line 109
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}{s}\n", .{"grep-P rc=", sh2Env("?")});
    try stdout.flush(); // line 111
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}\n", .{"AbCdEf"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"tr", "[:lower:]", "[:upper:]"});
    try stdout.print("{s}\n", .{"  spaced  "});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"tr", "-s", " "});
    // TODO(unsupported): pipeline → sh2.pipeline // line 115
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}", .{"short\nvery-long-line-here\n"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"wc", "-L"});
    // TODO(unsupported): pipeline → sh2.pipeline // line 117
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 117
    try stdout.print("{s}", .{"abcdef"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"tail", "-c", "2"});
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 118
    // TODO(unsupported): shopt → sh2.shopt // line 121
    // TODO(unsupported): shopt → sh2.shopt
    _ = sh2Shopt();
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (std.mem.eql(u8, "ABC", "=\"abc") and sh2Truthy(sh2Exec()));
    // TODO(unsupported): shopt → sh2.shopt // line 123
    // TODO(unsupported): shopt → sh2.shopt
    _ = sh2Shopt();
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = ((std.mem.eql(u8, "ABC", "=\"abc") and sh2Truthy(sh2Exec())) or sh2Truthy(sh2Exec()));
    _ = sh2Run(&[_][]const u8{"rm", "-f", "/tmp/gnu_both.txt", "/tmp/gnu_link", "/tmp/gnu_a.txt", "/tmp/gnu_b.txt"}); // line 127
    _ = &aa_one_;
    _ = &aa_three_;
    _ = &aa_two_;
    _ = &arr;
    _ = &y;
    _ = &z;
}
// 46 construct(s) lowered to TODO markers