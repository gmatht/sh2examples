const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
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

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var __sh2_casesubj_0: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

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
}
// 2 construct(s) lowered to TODO markers