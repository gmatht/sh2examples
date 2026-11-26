const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

fn sh2FileTest(flag: []const u8, p: []const u8) bool {
    const st = std.fs.cwd().statFile(p) catch return false;
    if (std.mem.eql(u8, flag, "-f")) return st.kind == .file;
    if (std.mem.eql(u8, flag, "-d")) return st.kind == .directory;
    if (std.mem.eql(u8, flag, "-e")) return true;
    if (std.mem.eql(u8, flag, "-s")) return st.size != 0;
    return false;
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2GetVar() []const u8 {
    sh2TODO("getVar");
    return "";
}

fn sh2TestStub() bool {
    sh2TODO("test");
    return false;
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    if (((std.mem.eql(u8, sh2GetVar(), sh2GetVar()) and sh2Truthy(sh2Exec())) and sh2FileTest("-f", sh2GetVar()))) {
        // TODO(unsupported): exec → sh2.exec
        // TODO(unsupported): exec → sh2.exec
        if ((std.mem.eql(u8, sh2GetVar(), "~\"value") or sh2Truthy(sh2Exec()))) {
            if (sh2Test()) {
                try stdout.print("{s}\n", .{"Deeply nested condition met"});
    try stdout.flush();
            }
        }
    }
}
// 4 construct(s) lowered to TODO markers