@ECHO OFF

pushd %~dp0

REM Command line options can be passed by setting the SPHINXOPTS variable.
if "%SPHINXBUILD%" == "" (
	where sphinx-build >nul 2>nul
	if errorlevel 1 (
		set SPHINXBUILD=python -m sphinx.cmd.build
	) else (
		set SPHINXBUILD=sphinx-build
	)
)

if "%SPHINXINTL%" == "" (
	where sphinx-intl >nul 2>nul
	if errorlevel 1 (
		set SPHINXINTL=python -m sphinx_intl
	) else (
		set SPHINXINTL=sphinx-intl
	)
)

if "%SPHINXOPTS%" == "" set SPHINXOPTS=-W --keep-going

set SOURCEDIR=.
set BUILDDIR=_build

if "%1" == "" goto help

if "%1" == "gettext" (
	%SPHINXBUILD% -b gettext %SOURCEDIR% %BUILDDIR%/gettext %SPHINXOPTS% %O%
	goto end
)

if "%1" == "gettext-update" (
	%SPHINXBUILD% -b gettext %SOURCEDIR% %BUILDDIR%/gettext %SPHINXOPTS% %O%
	%SPHINXINTL% update -p %BUILDDIR%/gettext -l en
	goto end
)

if "%1" == "html-ja" (
	%SPHINXBUILD% -b html %SOURCEDIR% %BUILDDIR%/html/ja -D language=ja %SPHINXOPTS% %O%
	goto end
)

if "%1" == "html-en" (
	%SPHINXBUILD% -b html %SOURCEDIR% %BUILDDIR%/html/en -D language=en %SPHINXOPTS% %O%
	goto end
)

if "%1" == "html-all" (
	%SPHINXBUILD% -b html %SOURCEDIR% %BUILDDIR%/html/ja -D language=ja %SPHINXOPTS% %O%
	%SPHINXBUILD% -b html %SOURCEDIR% %BUILDDIR%/html/en -D language=en %SPHINXOPTS% %O%
	goto end
)

%SPHINXBUILD% -M %1 %SOURCEDIR% %BUILDDIR% %SPHINXOPTS% %O%
goto end



:help
%SPHINXBUILD% -M help %SOURCEDIR% %BUILDDIR% %SPHINXOPTS% %O%

:end
popd
