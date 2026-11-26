const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
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

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var cpu: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    if (matchGlob(sh2Env("arch"), "i*86") or std.mem.eql(u8, sh2Env("arch"), "x86_64")) { // line 3
        cpu = sh2Env("basic_machine");
    }
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}{s}", .{"cpu", "=[", sh2Param(), "]\n"});
    try stdout.flush();
    _ = &cpu;
}
// 2 construct(s) lowered to TODO markers