const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
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

fn sh2GetVar() []const u8 {
    sh2TODO("getVar");
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


fn try_(init: std.process.Init, __args: []const []const u8) ![]const u8 {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    var arg: []const u8 = "";
    var rc: []const u8 = "";
    var stderr: []const u8 = "";
    var stdout_: []const u8 = "";
    _ = sh2Run(&[_][]const u8{"local", "desc=", (if (__args.len >= 1) __args[1 - 1] else "")});
    _ = sh2Run(&[_][]const u8{"shift"});
    // TODO(unsupported): listVar → sh2.listVar
    // TODO(unsupported): listVar → sh2.listVar
    _ = sh2Run(&[_][]const u8{"local", [_] []const u8{sh2ListVar()}});
    try stdout.print("{s}{s}{s}\n", .{"==== ", sh2Env("desc"), " ===="});
    try stdout.flush();
    try stdout.print("{s}", .{"  $ "});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    {
        const sh2_items0 = [_][]const u8{sh2Param()};
        for (sh2_items0[0..]) |sh2_v1| {
            arg = sh2_v1;
            if (std.mem.eql(u8, arg, "~[[:space:]]")) {
                try stdout.print("{s}{s}{s}", .{"'", arg, "' "});
    try stdout.flush();
            } else {
                try stdout.print("{s}{s}", .{arg, " "});
    try stdout.flush();
            }
        }
    }
    try stdout.print("{s}\n", .{""});
    try stdout.flush();
    // declare stdout_
    // declare stderr
    // declare rc
    stdout_ = sh2Capture();
    rc = sh2Env("?");
    stderr = sh2Capture();
    _ = sh2Run(&[_][]const u8{"rm", "-f", std.fmt.allocPrint(std.heap.page_allocator, "/tmp/realpath_stderr.{s}", .{sh2Env("$")}) catch ""});
    if (!std.mem.eql(u8, stdout_, "")) {
        if (std.mem.eql(u8, stdout_, sh2GetVar())) {
            try stdout.print("{s}", .{"  stdout (NUL\xe2\x80\x91terminated): "});
    try stdout.flush();
            // TODO(unsupported): pipeline → sh2.pipeline
            // TODO(unsupported): pipeline → sh2.pipeline
            _ = sh2Pipeline();
            try stdout.print("{s}", .{"\n"});
    try stdout.flush();
        } else {
            try stdout.print("{s}{s}{s}", .{"  stdout: ", stdout_, "\n"});
    try stdout.flush();
        }
    } else {
        try stdout.print("{s}", .{"  stdout: (empty)\n"});
    try stdout.flush();
    }
    if (!std.mem.eql(u8, stderr, "")) {
        try stdout.print("{s}{s}{s}", .{"  stderr: ", stderr, "\n"});
    try stdout.flush();
    }
    try stdout.print("{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}", .{" ", " ", "e", "x", "i", "t", " ", "c", "o", "d", "e", ":", " ", rc, "\\", "n", "\\", "n"});
    try stdout.flush();
}
pub fn main(init: std.process.Init) !void {

    var rc_zero: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    _ = sh2Run(&[_][]const u8{"set", "-o", "nounset"}); // line 9
    _ = sh2Run(&[_][]const u8{"try", "realpath (default, --physical) on a simple file", "realpath", "/bin"}); // line 64
    _ = sh2Run(&[_][]const u8{"try", "realpath on a two\xe2\x80\x91hop symlink chain: /usr/bin/vi \xe2\x86\x92 /etc/alternatives/vi \xe2\x86\x92 /usr/bin/vim.basic", "realpath", "/usr/bin/vi"}); // line 67
    _ = sh2Run(&[_][]const u8{"try", "realpath on a relative symlink: /usr/local/bin/pi", "realpath", "/usr/local/bin/pi"}); // line 70
    _ = sh2Run(&[_][]const u8{"try", "realpath on a regular file (no symlinks)", "realpath", "/etc/hostname"}); // line 73
    _ = sh2Run(&[_][]const u8{"try", "realpath on a directory with .. component", "realpath", "/tmp/.."}); // line 76
    _ = sh2Run(&[_][]const u8{"try", "--canonicalize-existing on an existing path", "realpath", "--canonicalize-existing", "/usr/bin/sh"}); // line 85
    // TODO(unsupported): exec → sh2.exec // line 88
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Exec()) or true);
    _ = sh2Run(&[_][]const u8{"try", "--canonicalize-missing on a path with a non\xe2\x80\x91existent leaf", "realpath", "--canonicalize-missing", "/tmp/no_such_file_xyzzy"}); // line 96
    _ = sh2Run(&[_][]const u8{"try", "--canonicalize-missing on a completely imaginary path", "realpath", "--canonicalize-missing", "/nonexistent/deeply/missing/file"}); // line 99
    _ = sh2Run(&[_][]const u8{"try", "--canonicalize-missing on a path with .. and non\xe2\x80\x91existent parts", "realpath", "--canonicalize-missing", "/tmp/../nonexistent/../foo"}); // line 102
    _ = sh2Run(&[_][]const u8{"try", "--logical: /bin/..  (bin is a symlink, logical resolves .. before following it)", "realpath", "--logical", "/bin/.."}); // line 110
    _ = sh2Run(&[_][]const u8{"try", "default (--physical) for comparison: /bin/..", "realpath", "--physical", "/bin/.."}); // line 113
    _ = sh2Run(&[_][]const u8{"try", "--physical: /bin/..  (explicit, same as default)", "realpath", "--physical", "/bin/.."}); // line 121
    _ = sh2Run(&[_][]const u8{"try", "--strip (no symlink expansion) on /usr/bin/vi", "realpath", "--strip", "/usr/bin/vi"}); // line 129
    _ = sh2Run(&[_][]const u8{"try", "--strip on /bin  (symlink /bin \xe2\x86\x92 usr/bin)", "realpath", "--strip", "/bin"}); // line 132
    _ = sh2Run(&[_][]const u8{"try", "compare: default (--physical) on /bin", "realpath", "/bin"}); // line 135
    _ = sh2Run(&[_][]const u8{"try", "--relative-to=/usr/bin for /usr/bin/sh", "realpath", "--relative-to=/usr/bin", "/usr/bin/sh"}); // line 143
    _ = sh2Run(&[_][]const u8{"try", "--relative-to=/tmp for /etc/hostname", "realpath", "--relative-to=/tmp", "/etc/hostname"}); // line 146
    _ = sh2Run(&[_][]const u8{"try", "--relative-to=/ for /etc/hostname", "realpath", "--relative-to=/", "/etc/hostname"}); // line 149
    _ = sh2Run(&[_][]const u8{"try", "--relative-base=/etc for /etc/hostname (below /etc \xe2\x86\x92 relative)", "realpath", "--relative-base=/etc", "/etc/hostname"}); // line 157
    _ = sh2Run(&[_][]const u8{"try", "--relative-base=/etc for /usr/bin/sh (not below /etc \xe2\x86\x92 absolute)", "realpath", "--relative-base=/etc", "/usr/bin/sh"}); // line 160
    _ = sh2Run(&[_][]const u8{"try", "--relative-base=/ with --relative-to=/tmp  (combined)", "realpath", "--relative-base=/", "--relative-to=/tmp", "/etc/hostname", "/usr/bin/sh"}); // line 163
    try stdout.print("{s}\n", .{"==== --zero with two paths ===="});
    try stdout.flush(); // line 173
    try stdout.print("{s}", .{"  $ realpath --zero /bin /usr/bin/sh\n"});
    try stdout.flush(); // line 174
    _ = sh2Run(&[_][]const u8{"realpath", "--zero", "/bin", "/usr/bin/sh"}); // line 175
    rc_zero = sh2Env("?"); // line 175
    try stdout.print("{s}", .{"  (raw output with NULs above; use od to verify):\n"});
    try stdout.flush(); // line 177
    try stdout.print("{s}", .{"  "});
    try stdout.flush(); // line 178
    // TODO(unsupported): pipeline → sh2.pipeline // line 178
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}", .{" ", " ", "e", "x", "i", "t", " ", "c", "o", "d", "e", ":", " ", rc_zero, "\\", "n", "\\", "n"});
    try stdout.flush(); // line 179
    // TODO(unsupported): exec → sh2.exec // line 188
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Exec()) or true);
    // TODO(unsupported): exec → sh2.exec // line 191
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Exec()) or true);
    try stdout.print("{s}\n", .{"=== All tests completed ==="});
    try stdout.flush();
}
// 14 construct(s) lowered to TODO markers