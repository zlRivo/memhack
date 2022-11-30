// injector.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include <Windows.h>
#include <string>
#include <TlHelp32.h>
#define MAX_PATH 260

DWORD GetProcID(const char* name)
{
    DWORD pid = 0;
    char pNameBuf[MAX_PATH] = { 0 };
    PROCESSENTRY32 entry;
    entry.dwSize = sizeof(PROCESSENTRY32);
    
    HANDLE snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

    if (Process32First(snapshot, &entry) == TRUE)
    {
        do
        {
            // Convert w_char to char
            wcstombs_s(NULL, pNameBuf, entry.szExeFile, MAX_PATH);
            // Compare proces names
            if (_stricmp(pNameBuf, name) == 0)
            {
                pid = entry.th32ProcessID;
                break;
            }
        } while (Process32Next(snapshot, &entry) == TRUE);
    }

    return pid;
}

int main(int argc, char* argv[])
{
    if (argc != 3) {
        std::cerr << "Usage: injector.exe <DLL> <PROCESS NAME>" << std::endl;
        exit(1);
    }

    DWORD pid = GetProcID(argv[2]);
    if (pid == 0)
    {
        std::cerr << "Could not find process ID! Exiting." << std::endl;
        exit(1);
    }

    std::cout << "Found Program ID : " << pid << std::endl;

    // Get the full name path of the dll
    char dllPath[MAX_PATH] = { 0 };
    GetFullPathNameA(argv[1], MAX_PATH, dllPath, NULL);

    // Ensure file exists
    LPWIN32_FIND_DATAA find_data;
    if (GetFileAttributesA(dllPath) == 0xFFFFFFFF)
    {
        std::cerr << "Could not find DLL. Exiting" << std::endl;
        exit(1);
    }

    std::cout << "DLL Path : " << dllPath << std::endl;

    // Open handle to target process
    HANDLE hProcess = OpenProcess(PROCESS_CREATE_THREAD | PROCESS_QUERY_INFORMATION | PROCESS_VM_OPERATION | PROCESS_VM_WRITE | PROCESS_VM_READ, FALSE, pid);

    // Allocate memory to write dll path in process
    LPVOID dllPathAlloc = VirtualAllocEx(hProcess, NULL, strlen(dllPath) + 1, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);

    // Write dll path
    WriteProcessMemory(hProcess, dllPathAlloc, dllPath, strlen(dllPath) + 1, NULL);

    // Run LoadLibraryA in remote process with dll path
    HANDLE hThread = CreateRemoteThread(hProcess, NULL, NULL, (LPTHREAD_START_ROUTINE)LoadLibraryA, dllPathAlloc, NULL, NULL);

    std::cout << "Successfully injected DLL into PID : " << pid << std::endl;

    WaitForSingleObject(hThread, INFINITE);
    CloseHandle(hThread);

    // Free allocated memory
    VirtualFreeEx(hProcess, dllPathAlloc, 0, MEM_RELEASE);
    CloseHandle(hProcess);

    return 0;
}

// Run program: Ctrl + F5 or Debug > Start Without Debugging menu
// Debug program: F5 or Debug > Start Debugging menu

// Tips for Getting Started: 
//   1. Use the Solution Explorer window to add/manage files
//   2. Use the Team Explorer window to connect to source control
//   3. Use the Output window to see build output and other messages
//   4. Use the Error List window to view errors
//   5. Go to Project > Add New Item to create new code files, or Project > Add Existing Item to add existing code files to the project
//   6. In the future, to open this project again, go to File > Open > Project and select the .sln file
