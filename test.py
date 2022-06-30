import elasticapmspeedups
import inspect
# should output lines 9 through 13 to stdout
print(elasticapmspeedups.read_lines_from_file('input', 10, 2))

# should print a list of dicts of frame info
print(elasticapmspeedups.walk_stack(inspect.currentframe()))
locals = getattr(inspect.currentframe(), "f_locals", {})
print(elasticapmspeedups.dictate(locals))