const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
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

fn sh2GetVar() []const u8 {
    sh2TODO("getVar");
    return "";
}


fn test_math(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    _ = __args;
    _ = sh2Run(&[_][]const u8{"local", "first_number=$1"});
    _ = sh2Run(&[_][]const u8{"local", "second_number=$2"});
    _ = sh2Run(&[_][]const u8{"local", "operation=$3"});
    if ((!!std.mem.eql(u8, sh2Env("operation"), "add"))) {
        return sh2IntStr((sh2ToInt(sh2GetVar()) + sh2ToInt(sh2GetVar())));
    } else if ((!!std.mem.eql(u8, sh2Env("operation"), "subtract"))) {
        return sh2IntStr((sh2ToInt(sh2GetVar()) - sh2ToInt(sh2GetVar())));
    } else if ((!!std.mem.eql(u8, sh2Env("operation"), "multiply"))) {
        return sh2IntStr((sh2ToInt(sh2GetVar()) * sh2ToInt(sh2GetVar())));
    } else {
        return std.fmt.allocPrint(std.heap.page_allocator, "Unknown operation: {s}", .{sh2Env("operation")}) catch "";
    }
}
fn test_strings(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    _ = __args;
    _ = sh2Run(&[_][]const u8{"local", "input_string=$1"});
    _ = sh2Run(&[_][]const u8{"local", "search_pattern=$2"});
    _ = sh2Run(&[_][]const u8{"local", "replacement=$3"});
    if ((!!std.mem.eql(u8, sh2Env("search_pattern"), "start"))) {
        return std.fmt.allocPrint(std.heap.page_allocator, "Replacing start of: {s} with: {s}", .{sh2Env("input_string"), sh2Env("replacement")}) catch "";
    } else if ((!!std.mem.eql(u8, sh2Env("search_pattern"), "end"))) {
        return std.fmt.allocPrint(std.heap.page_allocator, "Replacing end of: {s} with: {s}", .{sh2Env("input_string"), sh2Env("replacement")}) catch "";
    } else if ((!!std.mem.eql(u8, sh2Env("search_pattern"), "middle"))) {
        return std.fmt.allocPrint(std.heap.page_allocator, "Replacing middle of: {s} with: {s}", .{sh2Env("input_string"), sh2Env("replacement")}) catch "";
    } else {
        return std.fmt.allocPrint(std.heap.page_allocator, "Unknown pattern: {s} for string: {s}", .{sh2Env("search_pattern"), sh2Env("input_string")}) catch "";
    }
}
fn test_arrays(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    _ = __args;
    _ = sh2Run(&[_][]const u8{"local", "array_name=$1"});
    _ = sh2Run(&[_][]const u8{"local", "index=$2"});
    _ = sh2Run(&[_][]const u8{"local", "new_value=$3"});
    if ((!!std.mem.eql(u8, sh2Env("index"), "first"))) {
        return std.fmt.allocPrint(std.heap.page_allocator, "Setting first element of {s} to {s}", .{sh2Env("array_name"), sh2Env("new_value")}) catch "";
    } else if ((!!std.mem.eql(u8, sh2Env("index"), "last"))) {
        return std.fmt.allocPrint(std.heap.page_allocator, "Setting last element of {s} to {s}", .{sh2Env("array_name"), sh2Env("new_value")}) catch "";
    } else {
        return std.fmt.allocPrint(std.heap.page_allocator, "Setting element {s} of {s} to {s}", .{sh2Env("index"), sh2Env("array_name"), sh2Env("new_value")}) catch "";
    }
}
pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}\n", .{test_math(init, &[_][]const u8{ "10", "5", "add" })});
    try stdout.flush();
    try stdout.print("{s}\n", .{test_math(init, &[_][]const u8{ "10", "5", "multiply" })});
    try stdout.flush();
    try stdout.print("{s}\n", .{test_strings(init, &[_][]const u8{ "hello world", "start", "hi" })});
    try stdout.flush();
    try stdout.print("{s}\n", .{test_strings(init, &[_][]const u8{ "hello world", "end", "bye" })});
    try stdout.flush();
    try stdout.print("{s}\n", .{test_arrays(init, &[_][]const u8{ "my_array", "first", "new_value" })});
    try stdout.flush();
    try stdout.print("{s}\n", .{test_arrays(init, &[_][]const u8{ "my_array", "last", "final_value" })});
    try stdout.flush();
}