const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2Join() []const u8 {
    sh2TODO("join");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var first_half: [1024]i64 = undefined;
    var first_half_len: usize = 0;
    var last_half: [1024]i64 = undefined;
    var last_half_len: usize = 0;
    var middle: [1024]i64 = undefined;
    var middle_len: usize = 0;
    var numbers: [1024]i64 = undefined;
    var numbers_len: usize = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    numbers_len = 0; // line 4
    numbers[numbers_len] = 1;
    numbers_len += 1;
    numbers[numbers_len] = 2;
    numbers_len += 1;
    numbers[numbers_len] = 3;
    numbers_len += 1;
    numbers[numbers_len] = 4;
    numbers_len += 1;
    numbers[numbers_len] = 5;
    numbers_len += 1;
    numbers[numbers_len] = 6;
    numbers_len += 1;
    numbers[numbers_len] = 7;
    numbers_len += 1;
    numbers[numbers_len] = 8;
    numbers_len += 1;
    numbers[numbers_len] = 9;
    numbers_len += 1;
    numbers[numbers_len] = 10;
    numbers_len += 1;
    middle_len = 0; // line 5
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    middle[middle_len] = sh2ToInt(sh2Join());
    middle_len += 1;
    first_half_len = 0; // line 6
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    first_half[first_half_len] = sh2ToInt(sh2Join());
    first_half_len += 1;
    last_half_len = 0; // line 7
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    last_half[last_half_len] = sh2ToInt(sh2Join());
    last_half_len += 1;
    // TODO(unsupported): join → sh2.join // line 9
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"Middle: ", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): join → sh2.join // line 10
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"First half: ", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): join → sh2.join // line 11
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"Last half: ", sh2Join()});
    try stdout.flush();
    _ = &first_half;
    _ = &last_half;
    _ = &middle;
    _ = &numbers;
}
// 12 construct(s) lowered to TODO markers