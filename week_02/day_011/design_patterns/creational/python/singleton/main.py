from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Callable


class SingletonMeta(type):
    _instances = {}

    def __call__(cls, *args: str, **kwargs: str) -> Callable:
        if cls not in cls._instances:
            instance = super().__call__(*args, **kwargs)
            cls._instances[cls] = instance
        return cls._instances[cls]


class AppConfig(metaclass=SingletonMeta):
    def __init__(self) -> None:
        print("--- Loading Configuration ---")
        self.settings = {
            "api_url": "https://api.google.com",
            "debug_mode": True,
            "version": "1.0.2",
        }

    def get(self, key: str) -> str | None | float:
        return self.settings.get(key)


config_a = AppConfig()
print(f"Component A sees API: {config_a.get('api_url')}")  # ty:ignore[unresolved-attribute]

config_b = AppConfig()
print(f"Component B sees Debug: {config_b.get('debug_mode')}")  # ty:ignore[unresolved-attribute]


print(f"Did we reload the file? {'No' if config_a is config_b else 'Yes'}")
