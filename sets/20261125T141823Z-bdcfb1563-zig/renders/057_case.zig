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

fn sh2StrHasSuffix() []const u8 {
    sh2TODO("strHasSuffix");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var filename: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}\n", .{"=== Basic Case Statement Example ==="});
    try stdout.flush();
    if ((!!std.mem.eql(u8, sh2Env("1"), "start"))) {
        try stdout.print("{s}\n", .{"Starting the service..."});
    try stdout.flush();
    } else if ((!!std.mem.eql(u8, sh2Env("1"), "stop"))) {
        try stdout.print("{s}\n", .{"Stopping the service..."});
    try stdout.flush();
    } else if ((!!std.mem.eql(u8, sh2Env("1"), "restart"))) {
        try stdout.print("{s}\n", .{"Restarting the service..."});
    try stdout.flush();
    } else {
        try stdout.print("{s}{s}{s}\n", .{"Usage: ", sh2Env("0"), " {start|stop|restart}"});
    try stdout.flush();
        std.process.exit(@intCast(1));
    }
    try stdout.print("{s}\n", .{"=== Case Statement with Pattern Matching ==="});
    try stdout.flush();
    filename = sh2Env("2"); // line 28
    // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
    // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
    if (sh2Truthy(sh2StrHasSuffix())) {
        try stdout.print("{s}{s}\n", .{"Processing text file: ", filename});
    try stdout.flush();
    // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
    // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
    } else if (sh2Truthy(sh2StrHasSuffix())) {
        try stdout.print("{s}{s}\n", .{"Processing shell script: ", filename});
    try stdout.flush();
    // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
    // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
    } else if (sh2Truthy(sh2StrHasSuffix())) {
        try stdout.print("{s}{s}\n", .{"Processing Python file: ", filename});
    try stdout.flush();
    } else {
        try stdout.print("{s}{s}\n", .{"Unknown file type: ", filename});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{"=== Case Statement with Multiple Patterns ==="});
    try stdout.flush();
    if (((((!!std.mem.eql(u8, sh2Env("3"), "help")) or (!!std.mem.eql(u8, sh2Env("3"), "h"))) or (!!std.mem.eql(u8, sh2Env("3"), "-h"))) or (!!std.mem.eql(u8, sh2Env("3"), "--help")))) {
        try stdout.print("{s}\n", .{"Help information:"});
    try stdout.flush();
        try stdout.print("{s}\n", .{"  start  - Start the service"});
    try stdout.flush();
        try stdout.print("{s}\n", .{"  stop   - Stop the service"});
    try stdout.flush();
        try stdout.print("{s}\n", .{"  status - Show service status"});
    try stdout.flush();
    } else if (((((!!std.mem.eql(u8, sh2Env("3"), "status")) or (!!std.mem.eql(u8, sh2Env("3"), "s"))) or (!!std.mem.eql(u8, sh2Env("3"), "-s"))) or (!!std.mem.eql(u8, sh2Env("3"), "--status")))) {
        try stdout.print("{s}\n", .{"Service status: Running"});
    try stdout.flush();
    } else {
        try stdout.print("{s}{s}\n", .{"Unknown option: ", sh2Env("3")});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{"=== Case Statement with Character Classes ==="});
    try stdout.flush();
    if (matchGlob(sh2Env("4"), "[0-9]")) { // line 65
        try stdout.print("{s}{s}\n", .{"Single digit: ", sh2Env("4")});
    try stdout.flush();
    } else if (matchGlob(sh2Env("4"), "[a-z]")) {
        try stdout.print("{s}{s}\n", .{"Lowercase letter: ", sh2Env("4")});
    try stdout.flush();
    } else if (matchGlob(sh2Env("4"), "[A-Z]")) {
        try stdout.print("{s}{s}\n", .{"Uppercase letter: ", sh2Env("4")});
    try stdout.flush();
    } else if (matchGlob(sh2Env("4"), "[0-9][0-9]")) {
        try stdout.print("{s}{s}\n", .{"Two digit number: ", sh2Env("4")});
    try stdout.flush();
    } else {
        try stdout.print("{s}{s}\n", .{"Other character: ", sh2Env("4")});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{"=== Case Statement with Default Action ==="});
    try stdout.flush();
    if ((!!std.mem.eql(u8, sh2Env("5"), "red"))) {
        try stdout.print("{s}\n", .{"Color is red"});
    try stdout.flush();
    } else if ((!!std.mem.eql(u8, sh2Env("5"), "green"))) {
        try stdout.print("{s}\n", .{"Color is green"});
    try stdout.flush();
    } else if ((!!std.mem.eql(u8, sh2Env("5"), "blue"))) {
        try stdout.print("{s}\n", .{"Color is blue"});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{"=== Case Statement with Commands ==="});
    try stdout.flush();
    if ((!!std.mem.eql(u8, sh2Env("6"), "ls"))) {
        _ = sh2Run(&[_][]const u8{"ls", "-l", "-a"});
    } else if ((!!std.mem.eql(u8, sh2Env("6"), "date"))) {
        _ = sh2Run(&[_][]const u8{"date"});
    } else if ((!!std.mem.eql(u8, sh2Env("6"), "pwd"))) {
        _ = sh2Run(&[_][]const u8{"pwd"});
    } else if ((!!std.mem.eql(u8, sh2Env("6"), "whoami"))) {
        _ = sh2Run(&[_][]const u8{"whoami"});
    } else {
        try stdout.print("{s}\n", .{"Available commands: ls, date, pwd, whoami"});
    try stdout.flush();
    }
}
// 6 construct(s) lowered to TODO markers