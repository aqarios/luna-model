/* LunaModel C API — compiler attribute macros. */
#ifndef LUNA_MODEL_ATTRIBUTES_H
#define LUNA_MODEL_ATTRIBUTES_H

#if defined(__GNUC__) || defined(__clang__)
#define LM_DEPRECATED __attribute__((deprecated))
#define LM_DEPRECATED_MSG(note) __attribute__((deprecated(note)))
#elif defined(_MSC_VER)
#define LM_DEPRECATED __declspec(deprecated)
#define LM_DEPRECATED_MSG(note) __declspec(deprecated(note))
#else
#define LM_DEPRECATED
#define LM_DEPRECATED_MSG(note)
#endif

#endif /* LUNA_MODEL_ATTRIBUTES_H */
