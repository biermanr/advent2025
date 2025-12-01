import pkgutil
import importlib

for loader, module_name, is_pkg in pkgutil.walk_packages(__path__):
    full_module_name = f"{__name__}.{module_name}"
    if not module_name.startswith("day"):
        continue
    try:
        submodule = importlib.import_module(full_module_name)
    except ModuleNotFoundError:
        pass