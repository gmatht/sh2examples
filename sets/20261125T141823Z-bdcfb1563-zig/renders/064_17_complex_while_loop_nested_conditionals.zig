const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2GetVar() []const u8 {
    sh2TODO("getVar");
    return "";
}

pub fn main(init: std.process.Init) !void {

    const shell: []const u8 = "";
    const uid: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    while (sh2Truthy(sh2Exec())) {
        if (((sh2ToInt(uid) > 1000) and !std.mem.eql(u8, shell, "/bin/false"))) {
            if (std.mem.eql(u8, sh2GetVar(), "~^/home/")) {
                try stdout.print("{s}{s}{s}{s}{s}{s}\n", .{"User: ", sh2Env("user"), " (UID: ", uid, ") - ", sh2Env("home")});
    try stdout.flush();
            }
        }
    }
}
// 2 construct(s) lowered to TODO markers