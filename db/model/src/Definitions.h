#pragma once

#ifdef DB_EXPORT
#define DB_CLASS    __declspec(dllexport)
#else
#define DB_CLASS    __declspec(dllimport)
#endif
