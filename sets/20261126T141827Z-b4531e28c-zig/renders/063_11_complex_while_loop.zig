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

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var __ps_tmp0: []const u8 = "";
    var counter: i64 = 0;
    var input_file: []const u8 = "";
    var max_lines: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    input_file = sh2Capture(); // line 4
    max_lines = 3; // line 5
    counter = 0; // line 6
    try stdout.print("{s}", .{"# comment\nhas $(sub) here\nplain line\n${param} line\n"});
    try stdout.flush(); // line 7
    sh2WriteFile(input_file, "", false);
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
    _ = sh2Run(&[_][]const u8{"rm", "-f", input_file}); // line 27
    _ = &max_lines;
}
// 6 construct(s) lowered to TODO markers