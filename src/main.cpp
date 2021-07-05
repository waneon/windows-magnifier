#include <magnification.h>
#include <windows.h>

#include <cstring>
#include <iostream>

#include "maginfo.h"
#include "version.h"

const int TIMER_MS = 8;

void Magnify(const MagInfo& i);
void UnMagnify(const MagInfo& i);

// int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR
// pCmdLine, int nCmdShow) {
int main(int argc, char* argv[]) {
    // print version
    if (argc == 2) {
        if (!strcmp(argv[1], "--version") || !strcmp(argv[1], "-v")) {
            std::cout << VERSION << "\n";

            return 0;
        }
    }

    // get mag info
    MagInfo i = {};
    bool magnified = false;

    // turn off console
    FreeConsole();

    // init mag
    if (!MagInitialize()) return 0;
    // set toggle key
    if (!RegisterHotKey(NULL, 0, MOD_ALT, '1')) return 0;
    if (!RegisterHotKey(NULL, 1, MOD_ALT, '2')) return 0;

    // check event loop
    MSG msg = {};
    while (GetMessage(&msg, NULL, 0, 0)) {
        switch (msg.message) {
            case WM_HOTKEY:
                if (msg.wParam == 0) {
                    if (magnified) {
                        KillTimer(NULL, 1);
                        UnMagnify(i);
                    } else {
                        SetTimer(NULL, 1, TIMER_MS, NULL);
                    }
                    magnified = !magnified;
                } else {
                    goto out;
                }
                break;
            case WM_TIMER:
                if (magnified) Magnify(i);
                break;
        }
    }
out:

    // un-set toggle key
    UnregisterHotKey(NULL, 1);
    UnregisterHotKey(NULL, 2);
    // un-init mag
    MagUninitialize();

    return 0;
}

void Magnify(const MagInfo& i) {
    POINT p = {};
    GetCursorPos(&p);

    int x = i.TransformX(p.x), y = i.TransformY(p.y);

    MagSetFullscreenTransform(i.GetMagFactor(), x, y);

    // Magnification input transform feature
    // uiAccess option should be enabled in manifest
    RECT rcDest;
    rcDest.left = 0;
    rcDest.top = 0;
    rcDest.right = GetSystemMetrics(SM_CXSCREEN);
    rcDest.bottom = GetSystemMetrics(SM_CYSCREEN);

    RECT rcSource;
    rcSource.left = x;
    rcSource.top = y;
    rcSource.right = rcSource.left + (int)(rcDest.right / i.GetMagFactor());
    rcSource.bottom = rcSource.top + (int)(rcDest.bottom / i.GetMagFactor());

    MagSetInputTransform(TRUE, &rcSource, &rcDest);
}

void UnMagnify(const MagInfo& i) { MagSetFullscreenTransform(1, 0, 0); }