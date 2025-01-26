import gc

import dotenv

dotenv.load_dotenv()
del dotenv
gc.collect()

from core import ai, core
from core.ai import (
    groq,
)
from core.core import (
    bot,
    asearch,
    main_message,
)

__all__ = [
    "ai",
    "bot",
    "core",
    "groq",
    "asearch",
    "main_message",
]