const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    if (std.mem.eql(u8, "abc", "a*")) { // line 6
        try stdout.print("{s}\n", .{"bad"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"literal-star-nomatch"});
    try stdout.flush();
    }
    if (std.mem.eql(u8, "xyz", "a*")) { // line 7
        try stdout.print("{s}\n", .{"bad"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"const-star-nomatch"});
    try stdout.flush();
    }
    if (std.mem.eql(u8, "abc", "a*")) {
        try stdout.print("{s}\n", .{"bad"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"var-star-nomatch"});
    try stdout.flush();
    }
    if (std.mem.eql(u8, "abc", "=\"a*")) { // line 11
        try stdout.print("{s}\n", .{"bad"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"db-bracket-nomatch"});
    try stdout.flush();
    }
    if (std.mem.eql(u8, "abc", "[ab]*")) { // line 13
        try stdout.print("{s}\n", .{"bad"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"bracket-literal-nomatch"});
    try stdout.flush();
    }
}