const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
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

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var ADAPTER: []const u8 = "";
    var ADAPTER3: []const u8 = "";
    var BGPID: []const u8 = "";
    var CPID: []const u8 = "";
    var DEFAULT_ADAPTER: []const u8 = "";
    var DOINGS: []const u8 = "";
    var ERRORS: []const u8 = "";
    var N: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    _ = sh2Run(&[_][]const u8{"cd", "/tmp"}); // line 32
    try std.fs.cwd().writeFile(.{ .sub_path = "grep_p_routes.txt", .data = "default via 10.0.0.1 dev eth0 proto dhcp src 10.0.0.1 metric 100\n" }); // line 35
    try std.fs.cwd().writeFile(.{ .sub_path = "grep_p_routes2.txt", .data = "default via 10.7.7.7 dev vpn_se proto static\n" }); // line 36
    try stdout.print("{s}\n", .{"-- functions.sh / softy: lookbehind -o extraction --"});
    try stdout.flush();
    DEFAULT_ADAPTER = sh2Capture();
    try stdout.print("{s}{s}{s}\n", .{"adapter=[", DEFAULT_ADAPTER, "]"});
    try stdout.flush(); // line 40
    try stdout.print("{s}\n", .{"-- jobs.sh: lookbehind -o + grep -v + head -1 --"});
    try stdout.flush();
    ADAPTER = sh2Capture();
    // TODO(unsupported): param → sh2.param // line 44
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}\n", .{"adapter2=[", sh2Param(), "]"});
    try stdout.flush();
    ADAPTER3 = sh2Capture();
    // TODO(unsupported): param → sh2.param // line 46
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}\n", .{"adapter3=[", sh2Param(), "]"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"-- snap-debug-info.sh: non-capturing group + \\s, -o --"});
    try stdout.flush();
    try std.fs.cwd().writeFile(.{ .sub_path = "grep_p_snap.txt", .data = "ID Status When\n1 Doing now\n2 Error now\n3 Doing now\n4 Error later\n" }); // line 49
    DOINGS = sh2Capture();
    try stdout.print("{s}{s}{s}\n", .{"doings=[", DOINGS, "]"});
    try stdout.flush(); // line 51
    ERRORS = sh2Capture();
    try stdout.print("{s}{s}{s}\n", .{"errors=[", ERRORS, "]"});
    try stdout.flush(); // line 53
    try stdout.print("{s}\n", .{"-- softy: FQDN validation pattern (kept as grep -P) --"});
    try stdout.flush();
    N = sh2Capture(); // line 62
    try stdout.print("{s}{s}\n", .{"fqdn-count=", N});
    try stdout.flush(); // line 63
    N = sh2Capture(); // line 64
    try stdout.print("{s}{s}\n", .{"ip-count=", N});
    try stdout.flush(); // line 65
    try stdout.print("{s}\n", .{"-- memtester.sh / terminateProcess.sh: pgrep -P (untouched) --"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"sleep", "0.01"}); // line 68
    BGPID = sh2Env("!"); // line 69
    CPID = sh2Capture(); // line 70
    // TODO(unsupported): param → sh2.param // line 71
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}\n", .{"pgrep-no-match=[", sh2Param(), "]"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"wait", sh2IntStr(BGPID)}); // line 72
    std.process.exit(@intCast(0)); // line 74
    _ = &ADAPTER;
    _ = &ADAPTER3;
    _ = &CPID;
}
// 6 construct(s) lowered to TODO markers