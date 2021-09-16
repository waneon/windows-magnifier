#ifndef UNICODE
#define UNICODE
#endif

#include <magnification.h>
#include <windows.h>

#include "Magnifier.h"

// Time for re-rendering magnification
const int TIMER_MS = 8;
const LPWSTR MAG_CLASS_NAME = L"Mag Window Class";

// Global magnifier
Magnifier mag{};
HWND hWndMag = NULL;

// Forward declarations
void RegisterMagWindowClass(HINSTANCE hInstance);
void CreateMagWindow(HINSTANCE hInstance);
LRESULT CALLBACK MagWndProc(HWND hWnd, UINT message, WPARAM wParam,
                            LPARAM lParam);

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR pCmdLine,
                   int nCmdShow) {
    // Check magnifier already running
    HANDLE hMutex = OpenMutex(MUTEX_ALL_ACCESS, 0, L"Windows_Magnifier.0");
    if (!hMutex) {
        hMutex = CreateMutex(0, 0, L"Windows_Magnifier.0");
    } else {
        MessageBox(NULL, L"magnifier is already running!", NULL, MB_OK);

        return 0;
    }

    // Create mag window
    CreateMagWindow(hInstance);

    // Create tray icon
    NOTIFYICONDATA nid = {};
    nid.cbSize = sizeof(nid);
    nid.hWnd = hWndMag;
    nid.uID = NULL;
    nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
    nid.uCallbackMessage = WM_USER;
    nid.hIcon = LoadIcon(hInstance, L"APP_ICON");
    lstrcpy(nid.szTip, L"Magni Settings");

    Shell_NotifyIcon(NIM_ADD, &nid);

    // Set toggle key
    if (!RegisterHotKey(hWndMag, 0, MOD_ALT, '1')) return 0;

    // Check event loop
    MSG msg = {};
    while (GetMessage(&msg, hWndMag, 0, 0)) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    // Destruct resources
    Shell_NotifyIcon(NIM_DELETE, &nid);
    UnregisterHotKey(hWndMag, 1);

    return 0;
}

LRESULT CALLBACK MagWndProc(HWND hWnd, UINT message, WPARAM wParam,
                            LPARAM lParam) {
    switch (message) {
        case WM_HOTKEY:
            if (mag.magnified) {
                KillTimer(NULL, 1);
                mag.unmagnify();
            } else {
                SetTimer(hWndMag, 1, TIMER_MS, NULL);
            }
            mag.magnified = !mag.magnified;
            return 0;
        case WM_TIMER:
            mag.magnify();
            return 0;
        case WM_USER:
            switch (lParam) {
                case WM_LBUTTONUP:
                    PostQuitMessage(0);
                    return 0;
                default:
                    break;
            }
    }

    return DefWindowProc(hWnd, message, wParam, lParam);
}

void CreateMagWindow(HINSTANCE hInstance) {
    RegisterMagWindowClass(hInstance);

    // Create mag window class as a message-only window
    hWndMag = CreateWindowEx(0, MAG_CLASS_NAME, L"Mag Window", 0, 0, 0, 0, 0,
                             HWND_MESSAGE, NULL, NULL, NULL);
}

void RegisterMagWindowClass(HINSTANCE hInstance) {
    WNDCLASSEX wcex = {};
    wcex.cbSize = sizeof(WNDCLASSEX);
    wcex.lpfnWndProc = MagWndProc;
    wcex.hInstance = hInstance;
    wcex.lpszClassName = MAG_CLASS_NAME;

    RegisterClassEx(&wcex);
}