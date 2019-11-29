#pragma once

#ifdef linux
#define DB_CLASS
#elif _WIN32
#ifdef DB_EXPORT
#define DB_CLASS __declspec(dllexport)
#else
#define DB_CLASS __declspec(dllimport)
#endif
#endif
