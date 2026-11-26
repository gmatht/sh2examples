const std = @import("std");

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    if ((!!std.mem.eql(u8, sh2Env("1"), "start"))) {
        try stdout.print("{s}\n", .{"Starting..."});
    try stdout.flush();
        if ((!!std.mem.eql(u8, sh2Env("2"), "fast"))) {
            try stdout.print("{s}\n", .{"Fast start"});
    try stdout.flush();
        } else if ((!!std.mem.eql(u8, sh2Env("2"), "slow"))) {
            try stdout.print("{s}\n", .{"Slow start"});
    try stdout.flush();
        }
    } else if ((!!std.mem.eql(u8, sh2Env("1"), "stop"))) {
        try stdout.print("{s}\n", .{"Stopping..."});
    try stdout.flush();
    } else if ((!!std.mem.eql(u8, sh2Env("1"), "restart"))) {
        try stdout.print("{s}\n", .{"Restarting..."});
    try stdout.flush();
        if ((!!std.mem.eql(u8, sh2Env("2"), "soft"))) {
            try stdout.print("{s}\n", .{"Soft restart"});
    try stdout.flush();
        } else if ((!!std.mem.eql(u8, sh2Env("2"), "hard"))) {
            try stdout.print("{s}\n", .{"Hard restart"});
    try stdout.flush();
        }
    }
}