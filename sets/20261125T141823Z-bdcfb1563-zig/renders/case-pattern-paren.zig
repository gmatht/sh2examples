const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
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

    var arch: []const u8 = "";
    var cpu: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    arch = "x86_64"; // line 3
    if (matchGlob(sh2IntStr(arch), "i*86") or std.mem.eql(u8, sh2IntStr(arch), "x86_64")) {
        cpu = "pc";
    } else {
        cpu = "unknown";
    }
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}{s}", .{"cpu", "=[", sh2Param(), "]\n"});
    try stdout.flush();
    arch = "arm64"; // line 10
    if (matchGlob(sh2IntStr(arch), "i*86") or std.mem.eql(u8, sh2IntStr(arch), "x86_64")) {
        cpu = "pc";
    } else {
        cpu = "unknown";
    }
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}{s}{s}", .{"unknown: ", "cpu", "=[", sh2Param(), "]\n"});
    try stdout.flush();
    _ = &cpu;
}
// 4 construct(s) lowered to TODO markers