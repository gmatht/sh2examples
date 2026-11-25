import java.io.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.*;
import java.util.*;

public class Sh2Program {
    static long __SH_RC = 0;
    static String __SH_T = "";
    static String __v___mf = "";
    static boolean __SH_NOCASE = false;
    static String __SH_CWD = System.getProperty("user.dir");
    static LinkedHashMap<String, String> __SH_EXPORTS = new LinkedHashMap<>();
    static List<String> __SH_ARGV = new ArrayList<>();
    static Scanner __SH_IN = new Scanner(System.in);


    public static void main(String[] args) throws Exception {
    for (String __a : args) __SH_ARGV.add(__a);
    if ((!(!shEq(shArg(0), "a")))) {
        if ((!(!shEq(shArg(1), "x")))) {
            System.out.println("x");
            __SH_RC = 0;
        }
    } else if ((!(!shEq(shArg(0), "b")))) {
        System.out.println("b");
        __SH_RC = 0;
    }
    System.exit((int) __SH_RC);
}
    static String shCwd() { return __SH_CWD; }
    static String shAbs(String p) {
        if (p.isEmpty() || p.startsWith("/")) return p;
        return __SH_CWD + "/" + p;
    }
    static String shEnv(String n) {
        String v = System.getenv(n);
        return v == null ? "" : v;
    }

    static long shLen(String s) {
        return s.codePointCount(0, s.length());
    }

    static long shNum(String s) {
        try { return Long.parseLong(s.trim()); } catch (Exception e) {
            try { return (long) Double.parseDouble(s.trim()); } catch (Exception e2) { return 0L; }
        }
    }

    static String shArg(int i) {
        return (i >= 0 && i < __SH_ARGV.size()) ? __SH_ARGV.get(i) : "";
    }

    static long shShift(int n) {
        for (int i = 0; i < n && !__SH_ARGV.isEmpty(); i++) __SH_ARGV.remove(0);
        return 0L;
    }

    static long shCd(String dir) {
        if (dir.isEmpty()) dir = shEnv("HOME");
        File d = new File(dir);
        if (!d.isDirectory()) { System.err.println("cd: " + dir + ": No such file or directory"); return 1L; }
        System.setProperty("user.dir", d.getAbsolutePath());
        __SH_CWD = d.getAbsolutePath();
        return 0L;
    }

    static long shRun(String cmdline) {
        try {
            ProcessBuilder pb = new ProcessBuilder("bash", "-c", cmdline);
            pb.directory(new File(shCwd()));
            pb.environment().putAll(__SH_EXPORTS);
            pb.redirectErrorStream(false);
            pb.inheritIO();
            Process p = pb.start();
            int rc = p.waitFor();
            __SH_RC = rc;
            return rc;
        } catch (Exception e) {
            __SH_RC = 127;
            return 127L;
        }
    }

    static long shRunEval(String code) {
        return shRun(code);
    }

    static String shCapture(String cmdline) {
        try {
            ProcessBuilder pb = new ProcessBuilder("bash", "-c", cmdline);
            pb.directory(new File(shCwd()));
            pb.environment().putAll(__SH_EXPORTS);
            // stdin MUST be inherited: the default is a never-written pipe,
            // which deadlocks any captured command that reads stdin
            pb.redirectInput(ProcessBuilder.Redirect.INHERIT);
            pb.redirectErrorStream(false);
            Process p = pb.start();
            byte[] out = p.getInputStream().readAllBytes();
            p.getErrorStream().readAllBytes();
            int rc = p.waitFor();
            __SH_RC = rc;
            String s = new String(out, StandardCharsets.UTF_8);
            while (s.endsWith("\n") || s.endsWith("\r")) s = s.substring(0, s.length() - 1);
            return s;
        } catch (Exception e) {
            __SH_RC = 127;
            return "";
        }
    }

    static String shPrintf(String fmt, String[] args) {
        StringBuilder sb = new StringBuilder();
        // unescape backslash sequences in the literal first
        StringBuilder f = new StringBuilder();
        for (int i = 0; i < fmt.length(); i++) {
            char c = fmt.charAt(i);
            if (c == '\\' && i + 1 < fmt.length()) {
                char n = fmt.charAt(++i);
                switch (n) {
                    case 'n': f.append('\n'); break;
                    case 't': f.append('\t'); break;
                    case 'r': f.append('\r'); break;
                    case '\\': f.append('\\'); break;
                    default: f.append('\\').append(n);
                }
            } else f.append(c);
        }
        int specs = 0;
        for (int i = 0; i < f.length(); i++) {
            if (f.charAt(i) == '%' && i + 1 < f.length()) {
                char n = f.charAt(i + 1);
                if (n != '%') specs++;
                i++;
            }
        }
        int ai = 0;
        do {
            int used = 0;
            for (int i = 0; i < f.length(); i++) {
                char c = f.charAt(i);
                if (c == '%' && i + 1 < f.length()) {
                    char n = f.charAt(++i);
                    if (n == '%') { sb.append('%'); continue; }
                    // flags + width: [-0-9]* before the conversion char
                    int wstart = i;
                    while (i + 1 < f.length() && (f.charAt(i) == '-' || Character.isDigit(f.charAt(i)))) i++;
                    String fw = f.substring(wstart, i);
                    n = f.charAt(i);
                    boolean leftJust = fw.contains("-");
                    int width = 0;
                    try { String dg = fw.replace("-", ""); if (!dg.isEmpty()) width = Integer.parseInt(dg); }
                    catch (Exception e2) { width = 0; }
                    String arg = ai < args.length ? args[ai] : "";
                    used++;
                    if (n == 'd' || n == 'i') {
                        try { sb.append(Long.parseLong(arg.trim())); } catch (Exception e) { sb.append("0"); }
                        if (ai < args.length) ai++;
                    } else if (n == 's') {
                        StringBuilder padded = new StringBuilder(arg);
                        while (padded.length() < width) {
                            if (leftJust) padded.append(' '); else padded.insert(0, ' ');
                        }
                        sb.append(padded);
                        if (ai < args.length) ai++;
                    } else if (n == 'b') {
                        sb.append(arg.replace("\\n", "\n").replace("\\t", "\t"));
                        if (ai < args.length) ai++;
                    } else {
                        sb.append(arg);
                        if (ai < args.length) ai++;
                    }
                } else sb.append(c);
            }
            if (args.length == 0 || used == 0) break;
        } while (ai < args.length);
        return sb.toString();
    }

    static String shEscapes(String s) {
        return s.replace("\\n", "\n").replace("\\t", "\t").replace("\\\\", "\\");
    }

    static String shSubstr(String s, long off, long len) {
        int[] cp = s.codePoints().toArray();
        long start = off < 0 ? cp.length + off : off;
        if (start < 0) start = 0;
        if (start > cp.length) return "";
        long end = len < 0 ? cp.length : start + len;
        if (end > cp.length) end = cp.length;
        if (end <= start) return "";
        return new String(cp, (int) start, (int) (end - start));
    }

    static boolean shFnmatch(String pat, String s) {
        if (pat.indexOf('(') >= 0 && pat.matches(".*[!@?+*]\\(.*")) {
            try {
                StringBuilder re = new StringBuilder();
                shExtglobToRegex(pat, re);
                return s.matches(re.toString());
            } catch (Exception e) { /* fall through to plain fnmatch */ }
        }
        return shFnmatchImpl(pat.codePoints().toArray(), s, 0, 0);
    }

    /** Translate a bash extglob pattern (!(x) @(x) ?(x) +(x) *(x)) to Java regex. */
    static void shExtglobToRegex(String pat, StringBuilder out) {
        int i = 0;
        while (i < pat.length()) {
            char c = pat.charAt(i);
            if ((c == '!' || c == '@' || c == '?' || c == '+' || c == '*')
                && i + 1 < pat.length() && pat.charAt(i + 1) == '(') {
                int depth = 1; int j = i + 2;
                while (j < pat.length() && depth > 0) {
                    if (pat.charAt(j) == '(') depth++;
                    if (pat.charAt(j) == ')') depth--;
                    j++;
                }
                String inner = pat.substring(i + 2, j - 1);
                // translate alternatives separately
                java.util.List<String> alts = new ArrayList<>();
                StringBuilder cur = new StringBuilder(); int d2 = 0;
                for (char x : inner.toCharArray()) {
                    if (x == '(') d2++;
                    if (x == ')') d2--;
                    if (x == '|' && d2 == 0) { alts.add(cur.toString()); cur.setLength(0); }
                    else cur.append(x);
                }
                alts.add(cur.toString());
                StringBuilder altsRe = new StringBuilder();
                for (int k = 0; k < alts.size(); k++) {
                    if (k > 0) altsRe.append('|');
                    StringBuilder one = new StringBuilder();
                    shExtglobToRegex(alts.get(k), one);
                    altsRe.append("(?:").append(one).append(")");
                }
                switch (c) {
                    case '!': out.append("(?:(?!").append(altsRe).append(").)*"); break;
                    case '@': out.append("(?:").append(altsRe).append(")"); break;
                    case '?': out.append("(?:").append(altsRe).append(")?"); break;
                    case '+': out.append("(?:").append(altsRe).append(")+"); break;
                    default:  out.append("(?:").append(altsRe).append(")*");
                }
                i = j;
                continue;
            }
            switch (c) {
                case '*': out.append(".*"); break;
                case '?': out.append('.'); break;
                case '.': out.append("\\."); break;
                case '\\': out.append("\\\\"); break;
                case '[': out.append('['); break;
                case ']': out.append(']'); break;
                case '^': out.append("\\^"); break;
                case '$': out.append("\\$"); break;
                case '(': case ')': case '{': case '}':
                case '+': case '|':
                    if (c=='['||c==']') { out.append(c); break; }
                    out.append('\\').append(c); break;
                default: out.append(c);
            }
            i++;
        }
    }

    static boolean shFnmatchImpl(int[] p, String s, int pi, int si) {
        int[] t = s.codePoints().toArray();
        while (pi < p.length) {
            int c = p[pi];
            if (c == '*') {
                for (int k = si; k <= t.length; k++)
                    if (shFnmatchImpl(p, s, pi + 1, k)) return true;
                return false;
            } else if (c == '?') {
                if (si >= t.length) return false;
                pi++; si++;
            } else if (c == '[') {
                if (si >= t.length) return false;
                int j = pi + 1;
                boolean neg = false;
                if (j < p.length && (p[j] == '!' || p[j] == '^')) { neg = true; j++; }
                boolean matched = false;
                boolean first = true;
                while (j < p.length && (p[j] != ']' || first)) {
                    first = false;
                    if (j + 2 < p.length && p[j + 1] == '-' && p[j + 2] != ']') {
                        if (t[si] >= p[j] && t[si] <= p[j + 2]) matched = true;
                        j += 3;
                    } else {
                        if (t[si] == p[j]) matched = true;
                        j++;
                    }
                }
                if (j < p.length && p[j] == ']') j++;
                if (matched != neg) { pi = j + (j < p.length && p[j] == ']' ? 1 : 0); si++; pi = j; }
                else return false;
            } else if (c == '\\') {
                if (pi + 1 >= p.length || si >= t.length || p[pi + 1] != t[si]) return false;
                pi += 2; si++;
            } else {
                if (si >= t.length || t[si] != c) return false;
                pi++; si++;
            }
        }
        return si == t.length;
    }

    static String shStripShortestPrefix(String s, String pat) {
        int[] cp = s.codePoints().toArray();
        for (int k = 1; k <= cp.length; k++)
            if (shFnmatch(pat, new String(cp, 0, k))) return new String(cp, k, cp.length - k);
        return s;
    }
    static String shStripLongestPrefix(String s, String pat) {
        int[] cp = s.codePoints().toArray();
        for (int k = cp.length; k >= 1; k--)
            if (shFnmatch(pat, new String(cp, 0, k))) return new String(cp, k, cp.length - k);
        return s;
    }
    static String shStripShortestSuffix(String s, String pat) {
        int[] cp = s.codePoints().toArray();
        for (int k = cp.length - 1; k >= 0; k--)
            if (shFnmatch(pat, new String(cp, k, cp.length - k))) return new String(cp, 0, k);
        return s;
    }
    static String shStripLongestSuffix(String s, String pat) {
        int[] cp = s.codePoints().toArray();
        for (int k = 0; k <= cp.length; k++)
            if (shFnmatch(pat, new String(cp, k, cp.length - k))) return new String(cp, 0, k);
        return s;
    }

    static String shReplaceAll(String s, String pat, String rep) {
        StringBuilder re = new StringBuilder();
        for (char c : pat.toCharArray()) {
            if ("\\.[]{}()*+-?^$|".indexOf(c) >= 0) re.append('\\');
            re.append(c);
        }
        return s.replaceAll(re.toString(), java.util.regex.Matcher.quoteReplacement(rep));
    }
    static String shReplaceFirst(String s, String pat, String rep) {
        StringBuilder re = new StringBuilder();
        for (char c : pat.toCharArray()) {
            if ("\\.[]{}()*+-?^$|".indexOf(c) >= 0) re.append('\\');
            re.append(c);
        }
        return s.replaceFirst(re.toString(), java.util.regex.Matcher.quoteReplacement(rep));
    }

    static String shUpperAll(String s) { return s.toUpperCase(); }
    static String shLowerAll(String s) { return s.toLowerCase(); }
    static String shUpperFirst(String s) {
        if (s.isEmpty()) return s;
        int fc = s.codePointAt(0);
        return new String(Character.toChars(Character.toUpperCase(fc))) + s.substring(Character.charCount(fc));
    }
    static String shLowerFirst(String s) {
        if (s.isEmpty()) return s;
        int fc = s.codePointAt(0);
        return new String(Character.toChars(Character.toLowerCase(fc))) + s.substring(Character.charCount(fc));
    }

    static String shBasename(String s) {
        while (s.endsWith("/") && s.length() > 1) s = s.substring(0, s.length() - 1);
        // An all-slash input ("/" or "//") basenames to "/" — the naive
        // substring(1) would yield "" (verified against bash).
        if (s.equals("/")) return "/";
        int i = s.lastIndexOf('/');
        return i < 0 ? s : s.substring(i + 1);
    }

    static String shDirname(String s) {
        // GNU: strip trailing '/'s first, then the last component; a
        // pathless word is "."; the prefix is re-stripped and "" -> "/".
        // (The naive lastIndexOf-only form got "a/b/" -> "a/b" wrong.)
        while (s.length() > 1 && s.endsWith("/")) s = s.substring(0, s.length() - 1);
        int i = s.lastIndexOf('/');
        if (i < 0) return ".";
        String d = s.substring(0, i);
        while (d.length() > 1 && d.endsWith("/")) d = d.substring(0, d.length() - 1);
        return d.isEmpty() ? "/" : d;
    }

    static long shDiv(long a, long b) {
        if (b == 0) { System.err.println("division by 0"); System.exit(1); }
        return a / b;
    }
    static long shMod(long a, long b) {
        if (b == 0) { System.err.println("division by 0"); System.exit(1); }
        return a % b;
    }

    static long shPow(long a, long b) {
        long r = 1;
        for (long i = 0; i < b; i++) r *= a;
        return r;
    }

    static boolean shFtype(String kind, String path) {
        File f = new File(shAbs(path));
        switch (kind) {
            case "f": return f.isFile();
            case "d": return f.isDirectory();
            case "e": return f.exists();
            case "s": return f.isFile() && f.length() > 0;
            case "r": return f.canRead();
            case "w": return f.canWrite();
            case "x": return f.canExecute();
            case "L": try { return !f.getCanonicalPath().equals(f.getAbsolutePath()); }
                      catch (Exception e) { return false; }
            default: return false;
        }
    }
    static long shMtime(String path) {
        File f = new File(shAbs(path));
        return f.exists() ? f.lastModified() : -1L;
    }

    static String shReadln() {
        try {
            if (!__SH_IN.hasNextLine()) return null;
            return __SH_IN.nextLine();
        } catch (Exception e) { return null; }
    }

    static List<String> shSplit(long v) { return shSplit(String.valueOf(v)); }
    static List<String> shSplit(String s) {
        if (s.isEmpty()) return new ArrayList<>();
        List<String> out = new ArrayList<>(Arrays.asList(s.split("[ \\t\\n]+")));
        // trailing separators produce a trailing "" in Java's split
        while (!out.isEmpty() && out.get(out.size() - 1).isEmpty()) out.remove(out.size() - 1);
        return out;
    }

    static void shAset(List<String> a, long idx, String val) {
        while (a.size() <= idx) a.add("");
        a.set((int) idx, val);
    }

    static String shAget(List<String> a, long idx) {
        long i = idx < 0 ? a.size() + idx : idx;
        if (i < 0 || i >= a.size()) return "";
        return a.get((int) i);
    }

    static void shWriteFile(String path, String content, boolean append) throws IOException {
        if (append) Files.write(Paths.get(shAbs(path)), content.getBytes(StandardCharsets.UTF_8),
                                StandardOpenOption.CREATE, StandardOpenOption.APPEND);
        else Files.write(Paths.get(shAbs(path)), content.getBytes(StandardCharsets.UTF_8));
    }

    static PrintStream shSwapOut(PrintStream ps) {
        PrintStream old = System.out;
        old.flush();
        System.setOut(ps);
        return old;
    }
    static void shRestoreOut(PrintStream ps) { System.out.flush(); System.setOut(ps); }

    static PrintStream shSwapErr(PrintStream ps) {
        PrintStream old = System.err;
        old.flush();
        System.setErr(ps);
        return old;
    }
    static void shRestoreErr(PrintStream ps) { System.err.flush(); System.setErr(ps); }
    static PrintStream shNullStream() { return new PrintStream(OutputStream.nullOutputStream()); }

    static Scanner shSwapIn(Scanner sc) {
        Scanner old = __SH_IN;
        __SH_IN = sc;
        return old;
    }
    static void shRestoreIn(Scanner sc) { __SH_IN = sc; }

    static String shSnapshot() { return ""; }
    static void shRestore(String s) {}

    static boolean shRegex(String pat, String flags) { return false; }

    static long shContains(String hay, String needle) {
        return hay.contains(needle) ? 1L : 0L;
    }

    static String shQuote(String w) {
        if (w.isEmpty()) return "''";
        boolean plain = true;
        for (char c : w.toCharArray())
            if (!(Character.isLetterOrDigit(c) || "_-./=:@%^+,~".indexOf(c) >= 0)) { plain = false; break; }
        if (plain) return w;
        return "'" + w.replace("'", "'\\''") + "'";
    }

    static boolean shDeclared(String n) { return true; }

    static String shErrIfEmpty(String v, String msg) {
        if (v.isEmpty()) { System.err.println(msg); System.exit(1); }
        return v;
    }

    static boolean shRegexMatch(String s, String re) {
        try { return s.matches(re); } catch (Exception e) { return false; }
    }

    static boolean shEq(String a, String b) {
        return __SH_NOCASE ? a.equalsIgnoreCase(b) : a.equals(b);
    }

    static boolean shTestRc(boolean b) {
        __SH_RC = b ? 0 : 1;
        return b;
    }

    static String shDefinedOr(java.util.function.Supplier<String> subject, java.util.function.Supplier<String> dflt) {
        String v = subject.get();
        return v.isEmpty() ? dflt.get() : v;
    }

    static String shAslice(List<String> a, long off, long len) {
        long start = off < 0 ? a.size() + off : off;
        if (start < 0) start = 0;
        long end = len < 0 ? a.size() : start + len;
        if (end > a.size()) end = a.size();
        if (end <= start) return "";
        return String.join(" ", a.subList((int) start, (int) end));
    }

    // `wc -w`: whitespace-delimited word count (runs of whitespace are ONE
    // separator; leading/trailing whitespace never counts). The transition
    // scan needs no allocation and is exact for every input.
    static long shWcWords(String s) {
        long n = 0;
        boolean in = false;
        for (int i = 0; i < s.length(); i++) {
            if (Character.isWhitespace(s.charAt(i))) { in = false; }
            else if (!in) { in = true; n++; }
        }
        return n;
    }

    static String shCutField(String s, String delim, int[] fields) {
        if (!s.contains(delim)) return s;
        String[] parts = s.split(java.util.regex.Pattern.quote(delim), -1);
        java.util.Arrays.sort(fields);
        StringBuilder out = new StringBuilder();
        boolean first = true;
        for (int f : fields) {
            if (f < 0 || f >= parts.length) continue;
            if (!first) out.append(delim);
            out.append(parts[f]);
            first = false;
        }
        return out.toString();
    }

    static String shRegSub(String s, String pat, String rep) {
        return shRegSubImpl(s, pat, rep, false);
    }
    static String shRegSubAll(String s, String pat, String rep) {
        return shRegSubImpl(s, pat, rep, true);
    }
    static String shRegSubImpl(String s, String pat, String rep, boolean global) {
        java.util.regex.Pattern p = java.util.regex.Pattern.compile(pat);
        java.util.regex.Matcher m = p.matcher(s);
        if (!m.find()) return s;
        StringBuilder out = new StringBuilder();
        int last = 0;
        int start = m.start();
        out.append(s, 0, start);
        do {
            out.append(shExpandSed(m, rep));
            last = m.end();
            if (!global) break;
        } while (m.find());
        out.append(s, last, s.length());
        return out.toString();
    }
    static String shExpandSed(java.util.regex.Matcher m, String rep) {
        StringBuilder b = new StringBuilder();
        for (int i = 0; i < rep.length(); i++) {
            char c = rep.charAt(i);
            if (c == '&') { b.append(m.group()); continue; }
            if (c == '\\' && i + 1 < rep.length()) {
                char d = rep.charAt(++i);
                if (d >= '1' && d <= '9') {
                    int g = d - '0';
                    String v = g <= m.groupCount() ? m.group(g) : null;
                    b.append(v == null ? "" : v);
                } else { b.append(d); }
                continue;
            }
            b.append(c);
        }
        return b.toString();
    }


    static long shRegCount(String s, String pat) {
        java.util.regex.Matcher m = java.util.regex.Pattern.compile(pat).matcher(s);
        long n = 0;
        while (m.find()) n++;
        return n;
    }

    static String shTakeLines(String s, int n) {
        if (n <= 0) return "";
        String[] l = s.split("\\n", -1);
        StringBuilder b = new StringBuilder();
        for (int i = 0; i < l.length && i < n; i++) { if (i > 0) b.append("\n"); b.append(l[i]); }
        return b.toString();
    }
    static String shTakeLinesEnd(String s, int n) {
        if (n <= 0) return "";
        String[] l = s.split("\\n", -1);
        StringBuilder b = new StringBuilder();
        for (int i = Math.max(0, l.length - n); i < l.length; i++) { if (i > Math.max(0, l.length - n)) b.append("\n"); b.append(l[i]); }
        return b.toString();
    }
    static String shTakeBytes(String s, int n) {
        return n <= 0 ? "" : s.substring(0, Math.min(n, s.length()));
    }
    static String shTakeBytesEnd(String s, int n) {
        return n <= 0 ? "" : s.substring(Math.max(0, s.length() - n));
    }

    // `tr SET1 SET2` (translate), `tr -d SET` (delete), `tr -s SET`
    // (squeeze). SET2 shorter than SET1 repeats its LAST char (bash); an
    // empty SET2 means "no translation" for -s.
    static String shTrTranslate(String s, String from, String to) {
        StringBuilder b = new StringBuilder();
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            int k = from.indexOf(c);
            if (k < 0) { b.append(c); continue; }
            b.append(to.isEmpty() ? c : to.charAt(Math.min(k, to.length() - 1)));
        }
        return b.toString();
    }
    static String shTrDelete(String s, String set) {
        StringBuilder b = new StringBuilder();
        for (int i = 0; i < s.length(); i++) {
            if (set.indexOf(s.charAt(i)) < 0) b.append(s.charAt(i));
        }
        return b.toString();
    }
    static String shTrSqueeze(String s, String set) {
        StringBuilder b = new StringBuilder();
        char prev = 0;
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == prev && set.indexOf(c) >= 0) continue;
            b.append(c);
            prev = c;
        }
        return b.toString();
    }
    // `xargs`-style trim: remove leading/trailing whitespace AND squeeze
    // internal whitespace runs to one space (the bare-xargs contract).
    static String shXargsTrim(String s) {
        StringBuilder b = new StringBuilder();
        boolean spaced = false, started = false;
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (Character.isWhitespace(c)) { spaced = true; continue; }
            if (started && spaced) b.append(' ');
            b.append(c);
            started = true;
            spaced = false;
        }
        return b.toString();
    }
    // CutsetTrim: strip ALL leading/trailing chars present in the cutset
    // (side: "left"/"right" = one end only, else both).
    static String shCutsetTrim(String s, String cut, String side) {
        int a = 0, z = s.length();
        if (side == null || !side.equals("right")) {
            while (a < z && cut.indexOf(s.charAt(a)) >= 0) a++;
        }
        if (side == null || !side.equals("left")) {
            while (z > a && cut.indexOf(s.charAt(z - 1)) >= 0) z--;
        }
        return s.substring(a, z);
    }

}
