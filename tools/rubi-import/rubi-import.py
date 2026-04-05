# This is a helper script for formatting and adapting
# RUBI integration rules.

import sys
from pathlib import Path
import re


class Formatter:
    def __init__(self, file_path: Path):
        self._rules = file_path.read_text()

        self.add_empty_lines()
        self.replace_rule_type()
        self.replace_symbol_pattern()
        self.replace_mathematica_heads()
        self.replace_lists_with_tuples()
        self.add_line_breaks_and_swap_conditions()
        self.replace_binary_appl_with_op("EqQ", "==")
        self.replace_binary_appl_with_op("GtQ", ">")
        self.replace_binary_appl_with_op("LtQ", "<")

    @property
    def rules(self) -> str:
        return self._rules.strip()

    def add_empty_lines(self):
        s = re.sub(r"^Int\[", "\nInt[", self._rules, flags=re.MULTILINE)
        self._rules = s.strip()

    def replace_rule_type(self):
        self._rules = self._rules.replace(":=", ":>")

    def replace_symbol_pattern(self):
        self._rules = self._rules.replace("x_Symbol", "x_?IsSymbol")

    def replace_mathematica_heads(self):
        self._rules = (
            self._rules.replace("FreeQ[", "FreeOf[")
            .replace("IntegerQ", "IsInteger")
            .replace("RationalQ", "IsRational")
        )

    def replace_lists_with_tuples(self):
        self._rules = self._rules.replace("{", "(").replace("}", ")")

    def add_line_breaks_and_swap_conditions(self):
        lines = self._rules.splitlines()
        result = []
        for line in lines:
            rule_op_pos = line.find(":>")
            if rule_op_pos != -1:
                pattern = line[:rule_op_pos]
                rest = "    " + line[rule_op_pos:]
            else:
                result.append(line)
                continue

            cond_pos = rest.rfind("/;")
            if cond_pos != -1:
                repl = rest[:cond_pos]
                cond = "    " + rest[cond_pos:]
            else:
                result.append(line)
                continue

            result.extend((pattern.rstrip(), cond.rstrip(), repl.rstrip() + ";"))
        
        self._rules = "\n".join(result)

    def replace_binary_appl_with_op(self, head: str, op: str):
        head_pos = self._rules.find(f"{head}[")
        while head_pos != -1:
            if head_pos > 0 and self._rules[head_pos - 1].isalnum():
                head_pos = self._rules.find(f"{head}[", head_pos + 1)
                continue

            args = []
            nested_count = 0
            arg_start = head_pos + len(head) + 1
            terminated_at = None
            for i in range(arg_start, len(self._rules)):
                if self._rules[i] == "[":
                    nested_count += 1
                elif self._rules[i] == "]":
                    nested_count -= 1
                elif self._rules[i] == ",":
                    args.append(self._rules[arg_start:i])
                    arg_start = i + 1

                if nested_count == -1:
                    terminated_at = i + 1
                    args.append(self._rules[arg_start:i])
                    break

            assert len(args) == 2
            assert terminated_at is not None

            self._rules = (
                self._rules[:head_pos]
                + args[0].strip()
                + f" {op} "
                + args[1].strip()
                + " "
                + self._rules[terminated_at:]
            )

            head_pos = self._rules.find(f"{head}[", head_pos)


if __name__ == "__main__":
    p = Path(sys.argv[1])
    fmt = Formatter(p)
    p.write_text(fmt.rules)
    # print(fmt.rules)
