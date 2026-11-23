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

fn sh2StrHasSuffix() []const u8 {
    sh2TODO("strHasSuffix");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var d: []const u8 = "";
    var file: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    d = sh2Capture(); // line 6
    // TODO(unsupported): exec → sh2.exec // line 7
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Exec()) or sh2Truthy(sh2Exec()));
    _ = sh2Run(&[_][]const u8{"touch", "a.txt", "b.log", "c.dat", "x.yz"}); // line 8
    {
        const sh2_items0 = [_][]const u8{"\x01SH2GLOB\x01*.txt", "\x01SH2GLOB\x01*.log", "\x01SH2GLOB\x01*.dat"};
        for (sh2_items0[0..]) |sh2_v1| {
            file = sh2_v1;
            // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
            // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
            // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
            // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
            if ((sh2Truthy(sh2StrHasSuffix()) or sh2Truthy(sh2StrHasSuffix()))) {
                try stdout.print("{s}{s}\n", .{"Text file: ", file});
    try stdout.flush();
            // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
            // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
            } else if (sh2Truthy(sh2StrHasSuffix())) {
                try stdout.print("{s}{s}\n", .{"Data file: ", file});
    try stdout.flush();
            } else {
                try stdout.print("{s}{s}\n", .{"Other file: ", file});
    try stdout.flush();
            }
        }
    }
    _ = sh2Run(&[_][]const u8{"cd", "/"}); // line 16
    _ = sh2Run(&[_][]const u8{"rm", "-r", "-f", d}); // line 17
}
// 10 construct(s) lowered to TODO markers