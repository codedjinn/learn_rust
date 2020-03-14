@echo off

IF NOT EXIST .\target\debug\config\ GOTO CREATE_CONFIG_DIR

:COPY_FILES
echo Copying files
xcopy .\src\config\*.ron .\target\debug\config /r/y
xcopy .\src\prefab\*.ron .\target\debug\assets\prefab /r/y
GOTO DONE

:CREATE_CONFIG_DIR

echo Creating config directory

mkdir .\target\debug\config\

GOTO COPY_FILES

:DONE
echo Done
