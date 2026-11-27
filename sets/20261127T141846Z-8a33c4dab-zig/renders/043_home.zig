const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
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
    _ = init;

    // TODO(unsupported): exec → sh2.exec // line 1
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = ((std.mem.eql(u8, "~", sh2GetVar()) and sh2Truthy(sh2Exec())) or sh2Truthy(sh2Exec()));
    // TODO(unsupported): exec → sh2.exec // line 2
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = ((std.mem.eql(u8, "~/Documents", sh2GetVar()) and sh2Truthy(sh2Exec())) or sh2Truthy(sh2Exec()));
    // TODO(unsupported): exec → sh2.exec // line 3
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = ((std.mem.eql(u8, "~/Documents", sh2GetVar()) and sh2Truthy(sh2Exec())) or sh2Truthy(sh2Exec()));
}
// 12 construct(s) lowered to TODO markers