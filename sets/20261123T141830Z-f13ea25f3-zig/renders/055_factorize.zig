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


fn factorize(init: std.process.Init, __args: []const []const u8) ![]const u8 {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    _ = __args;
    var divisor: i64 = 0;
    _ = sh2Run(&[_][]const u8{"local", "n=$1"});
    divisor = 2;
    _ = sh2Run(&[_][]const u8{"local", "factors=", ""});
    try stdout.print("{s}{s}{s}", .{"Factors of ", sh2Env("n"), ": "});
    try stdout.flush();
    while ((sh2ToInt(sh2GetVar()) > 1)) {
        while ((@rem(sh2ToInt(sh2GetVar()), divisor) == 0)) {
            if ((!!std.mem.eql(u8, sh2GetVar(), ""))) {
                factors = sh2IntStr(divisor);
            } else {
                factors = std.fmt.allocPrint(std.heap.page_allocator, "{s} * {d}", .{factors, divisor}) catch "";
            }
            n = sh2IntStr(@divTrunc(sh2ToInt(n), divisor));
        }
        divisor = (divisor + 1);
        if (((divisor * divisor) > sh2ToInt(n))) {
            if ((sh2ToInt(n) > 1)) {
                if ((!!std.mem.eql(u8, factors, ""))) {
                    factors = sh2IntStr(n);
                } else {
                    factors = std.fmt.allocPrint(std.heap.page_allocator, "{s} * {s}", .{factors, n}) catch "";
                }
            }
            break;
        }
    }
    try stdout.print("{s}\n", .{factors});
    try stdout.flush();
}
pub fn main(init: std.process.Init) !void {

    const factors: []const u8 = "";
    const n: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}\n", .{"=== Number Factorization Examples ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"factorize", "12"}); // line 44
    _ = sh2Run(&[_][]const u8{"factorize", "28"}); // line 45
    _ = sh2Run(&[_][]const u8{"factorize", "100"}); // line 46
    _ = sh2Run(&[_][]const u8{"factorize", "12345"}); // line 47
    try stdout.print("{s}\n", .{"Factorization complete!"});
    try stdout.flush();
}