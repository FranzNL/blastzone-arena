Unicode True

!define APP_NAME "BlastZone Arena"
!define APP_EXE "blastzone-arena.exe"
!define APP_ID "com.blastzonearena.app"
!define OUT_FILE "blastzone-arena-windows-x86_64-setup.exe"

Name "${APP_NAME}"
OutFile "..\${OUT_FILE}"
InstallDir "$PROGRAMFILES64\${APP_NAME}"
InstallDirRegKey HKLM "Software\${APP_ID}" "Install_Dir"
RequestExecutionLevel admin
SetCompressor /SOLID lzma

Icon "icon.ico"
UninstallIcon "icon.ico"

Page directory
Page instfiles
UninstPage uninstConfirm
UninstPage instfiles

Section "Install"
  SetOutPath "$INSTDIR"
  File "blastzone-arena.exe"
  File "icon.ico"
  SetOutPath "$INSTDIR\assets"
  File /r "assets\*.*"

  WriteRegStr HKLM "Software\${APP_ID}" "Install_Dir" "$INSTDIR"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" \
    "DisplayName" "${APP_NAME}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" \
    "DisplayIcon" '"$INSTDIR\icon.ico"'
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" \
    "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" \
    "NoModify" 1
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" \
    "NoRepair" 1
  WriteUninstaller "$INSTDIR\uninstall.exe"

  CreateDirectory "$SMPROGRAMS\${APP_NAME}"
  CreateShortcut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" \
    "$INSTDIR\${APP_EXE}" "" "$INSTDIR\icon.ico" 0
  CreateShortcut "$DESKTOP\${APP_NAME}.lnk" \
    "$INSTDIR\${APP_EXE}" "" "$INSTDIR\icon.ico" 0
SectionEnd

Section "Uninstall"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}"
  DeleteRegKey HKLM "Software\${APP_ID}"

  Delete "$INSTDIR\${APP_EXE}"
  Delete "$INSTDIR\icon.ico"
  Delete "$INSTDIR\uninstall.exe"
  RMDir /r "$INSTDIR\assets"
  RMDir "$INSTDIR"

  Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
  RMDir "$SMPROGRAMS\${APP_NAME}"
  Delete "$DESKTOP\${APP_NAME}.lnk"
SectionEnd
