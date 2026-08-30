
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * This module provides access to environment variables that are injected _statically_ into your bundle at build time and are limited to _private_ access.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Static environment variables are [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env` at build time and then statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * **_Private_ access:**
 * 
 * - This module cannot be imported into client-side code
 * - This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured)
 * 
 * For example, given the following build time environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { ENVIRONMENT, PUBLIC_BASE_URL } from '$env/static/private';
 * 
 * console.log(ENVIRONMENT); // => "production"
 * console.log(PUBLIC_BASE_URL); // => throws error during build
 * ```
 * 
 * The above values will be the same _even if_ different values for `ENVIRONMENT` or `PUBLIC_BASE_URL` are set at runtime, as they are statically replaced in your code with their build time values.
 */
declare module '$env/static/private' {
	export const SVELTEKIT_FORK: string;
	export const NODE_ENV: string;
	export const INIT_CWD: string;
	export const NM: string;
	export const OBJDUMP: string;
	export const CXX_FOR_BUILD: string;
	export const QT_PLATFORM_PLUGIN: string;
	export const LC_PAPER: string;
	export const RANLIB: string;
	export const npm_command: string;
	export const npm_config_allow_scripts: string;
	export const LC_NUMERIC: string;
	export const CMAKE_PREFIX_PATH: string;
	export const npm_config_global_prefix: string;
	export const NVM_CD_FLAGS: string;
	export const XDG_DATA_DIRS: string;
	export const PWD: string;
	export const npm_config_init_module: string;
	export const npm_config_globalconfig: string;
	export const XDG_VTNR: string;
	export const LC_IDENTIFICATION: string;
	export const GPG_AGENT_INFO: string;
	export const _CONDA_EXE: string;
	export const CONDA_BUILD_SYSROOT: string;
	export const GCC_AR: string;
	export const _CE_CONDA: string;
	export const QT_QPA_PLATFORMTHEME: string;
	export const MAMBA_ROOT_PREFIX: string;
	export const LLAMA_UI_VERSION: string;
	export const GCC_RANLIB: string;
	export const SSH_AUTH_SOCK: string;
	export const LDFLAGS: string;
	export const LS_COLORS: string;
	export const CONDA_TOOLCHAIN_BUILD: string;
	export const NVM_DIR: string;
	export const GSETTINGS_SCHEMA_DIR_CONDA_BACKUP: string;
	export const LLAMA_UI_OUT_DIR: string;
	export const npm_package_json: string;
	export const DBUS_SESSION_BUS_ADDRESS: string;
	export const MAMBA_EXE: string;
	export const npm_config_userconfig: string;
	export const XDG_SESSION_TYPE: string;
	export const CC_FOR_BUILD: string;
	export const ELFEDIT: string;
	export const CXXFILT: string;
	export const GCC: string;
	export const LESSOPEN: string;
	export const MFLAGS: string;
	export const CONDA_SHLVL: string;
	export const LDFLAGS_LD: string;
	export const COLORFGBG: string;
	export const XDG_SEAT_PATH: string;
	export const CONDA_TOOLCHAIN_HOST: string;
	export const SIZE: string;
	export const DEBUG_CXXFLAGS: string;
	export const COLORTERM: string;
	export const XDG_CACHE_HOME: string;
	export const npm_config_user_agent: string;
	export const LD: string;
	export const NVM_BIN: string;
	export const GTK_CSD: string;
	export const COLOR: string;
	export const JAVA_LD_LIBRARY_PATH: string;
	export const npm_config_engine_strict: string;
	export const OPENCODE: string;
	export const LC_TIME: string;
	export const CFLAGS: string;
	export const ADDR2LINE: string;
	export const XDG_SEAT: string;
	export const CMAKE_ARGS: string;
	export const CPP: string;
	export const npm_package_version: string;
	export const LANGUAGE: string;
	export const DEBUGINFOD_URLS: string;
	export const OLDPWD: string;
	export const OBJCOPY: string;
	export const XAUTHORITY: string;
	export const npm_config_npm_version: string;
	export const NVM_INC: string;
	export const USER: string;
	export const OPENCODE_PID: string;
	export const LXQT_SESSION_CONFIG: string;
	export const BROWSER: string;
	export const PW_EXPERIMENTAL_SERVICE_WORKER_NETWORK_EVENTS: string;
	export const RSTUDIO_WHICH_R: string;
	export const npm_config_noproxy: string;
	export const WINDOWID: string;
	export const SHLVL: string;
	export const npm_node_execpath: string;
	export const _: string;
	export const HOME: string;
	export const ASDF_DIR: string;
	export const DESKTOP_SESSION: string;
	export const LC_MONETARY: string;
	export const npm_config_prefix: string;
	export const GPROF: string;
	export const AR: string;
	export const XDG_RUNTIME_DIR: string;
	export const XDG_SESSION_CLASS: string;
	export const READELF: string;
	export const AS: string;
	export const CPPFLAGS: string;
	export const AGENT: string;
	export const GXX: string;
	export const _CE_M: string;
	export const SAL_VCL_QT5_USE_CAIRO: string;
	export const JAVA_HOME: string;
	export const CPP_FOR_BUILD: string;
	export const GTK_OVERLAY_SCROLLING: string;
	export const XDG_SESSION_ID: string;
	export const GCC_NM: string;
	export const CXXFLAGS: string;
	export const SAL_USE_VCLPLUGIN: string;
	export const LOGNAME: string;
	export const SHELL: string;
	export const host_alias: string;
	export const PATH: string;
	export const npm_config_cache: string;
	export const TERM: string;
	export const NODE: string;
	export const CONDA_PROMPT_MODIFIER: string;
	export const XDG_CONFIG_HOME: string;
	export const SSH_AGENT_PID: string;
	export const npm_config_local_prefix: string;
	export const npm_package_name: string;
	export const XDG_MENU_PREFIX: string;
	export const XDG_CURRENT_DESKTOP: string;
	export const LC_MEASUREMENT: string;
	export const XML_CATALOG_FILES: string;
	export const LC_ADDRESS: string;
	export const npm_execpath: string;
	export const XDG_SESSION_PATH: string;
	export const npm_config_node_gyp: string;
	export const build_alias: string;
	export const CC: string;
	export const MAKELEVEL: string;
	export const DISPLAY: string;
	export const LANG: string;
	export const CONDA_PREFIX: string;
	export const DOTNET_BUNDLE_EXTRACT_BASE_DIR: string;
	export const _CONDA_ROOT: string;
	export const DEBUG_CFLAGS: string;
	export const EDITOR: string;
	export const LC_TELEPHONE: string;
	export const CONDA_PYTHON_EXE: string;
	export const MESON_ARGS: string;
	export const npm_config_ignore_scripts: string;
	export const XDG_SESSION_DESKTOP: string;
	export const STRINGS: string;
	export const XDG_DATA_HOME: string;
	export const MAKEFLAGS: string;
	export const npm_lifecycle_script: string;
	export const GSETTINGS_SCHEMA_DIR: string;
	export const LC_NAME: string;
	export const STRIP: string;
	export const npm_lifecycle_event: string;
	export const HOST: string;
	export const CONDA_EXE: string;
	export const BUILD: string;
	export const QT_ACCESSIBILITY: string;
	export const DEBUG_CPPFLAGS: string;
	export const CXX: string;
	export const LLAMA_BUILD_NUMBER: string;
	export const LESSCLOSE: string;
	export const XDG_CONFIG_DIRS: string;
	export const CONDA_DEFAULT_ENV: string;
	export const CONDA_BUILD_CROSS_COMPILATION: string;
}

/**
 * This module provides access to environment variables that are injected _statically_ into your bundle at build time and are _publicly_ accessible.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Static environment variables are [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env` at build time and then statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * **_Public_ access:**
 * 
 * - This module _can_ be imported into client-side code
 * - **Only** variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`) are included
 * 
 * For example, given the following build time environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { ENVIRONMENT, PUBLIC_BASE_URL } from '$env/static/public';
 * 
 * console.log(ENVIRONMENT); // => throws error during build
 * console.log(PUBLIC_BASE_URL); // => "http://site.com"
 * ```
 * 
 * The above values will be the same _even if_ different values for `ENVIRONMENT` or `PUBLIC_BASE_URL` are set at runtime, as they are statically replaced in your code with their build time values.
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to environment variables set _dynamically_ at runtime and that are limited to _private_ access.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Dynamic environment variables are defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`.
 * 
 * **_Private_ access:**
 * 
 * - This module cannot be imported into client-side code
 * - This module includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured)
 * 
 * > [!NOTE] In `dev`, `$env/dynamic` includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 * 
 * > [!NOTE] To get correct types, environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * >
 * > ```env
 * > MY_FEATURE_FLAG=
 * > ```
 * >
 * > You can override `.env` values from the command line like so:
 * >
 * > ```sh
 * > MY_FEATURE_FLAG="enabled" npm run dev
 * > ```
 * 
 * For example, given the following runtime environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * 
 * console.log(env.ENVIRONMENT); // => "production"
 * console.log(env.PUBLIC_BASE_URL); // => undefined
 * ```
 */
declare module '$env/dynamic/private' {
	export const env: {
		SVELTEKIT_FORK: string;
		NODE_ENV: string;
		INIT_CWD: string;
		NM: string;
		OBJDUMP: string;
		CXX_FOR_BUILD: string;
		QT_PLATFORM_PLUGIN: string;
		LC_PAPER: string;
		RANLIB: string;
		npm_command: string;
		npm_config_allow_scripts: string;
		LC_NUMERIC: string;
		CMAKE_PREFIX_PATH: string;
		npm_config_global_prefix: string;
		NVM_CD_FLAGS: string;
		XDG_DATA_DIRS: string;
		PWD: string;
		npm_config_init_module: string;
		npm_config_globalconfig: string;
		XDG_VTNR: string;
		LC_IDENTIFICATION: string;
		GPG_AGENT_INFO: string;
		_CONDA_EXE: string;
		CONDA_BUILD_SYSROOT: string;
		GCC_AR: string;
		_CE_CONDA: string;
		QT_QPA_PLATFORMTHEME: string;
		MAMBA_ROOT_PREFIX: string;
		LLAMA_UI_VERSION: string;
		GCC_RANLIB: string;
		SSH_AUTH_SOCK: string;
		LDFLAGS: string;
		LS_COLORS: string;
		CONDA_TOOLCHAIN_BUILD: string;
		NVM_DIR: string;
		GSETTINGS_SCHEMA_DIR_CONDA_BACKUP: string;
		LLAMA_UI_OUT_DIR: string;
		npm_package_json: string;
		DBUS_SESSION_BUS_ADDRESS: string;
		MAMBA_EXE: string;
		npm_config_userconfig: string;
		XDG_SESSION_TYPE: string;
		CC_FOR_BUILD: string;
		ELFEDIT: string;
		CXXFILT: string;
		GCC: string;
		LESSOPEN: string;
		MFLAGS: string;
		CONDA_SHLVL: string;
		LDFLAGS_LD: string;
		COLORFGBG: string;
		XDG_SEAT_PATH: string;
		CONDA_TOOLCHAIN_HOST: string;
		SIZE: string;
		DEBUG_CXXFLAGS: string;
		COLORTERM: string;
		XDG_CACHE_HOME: string;
		npm_config_user_agent: string;
		LD: string;
		NVM_BIN: string;
		GTK_CSD: string;
		COLOR: string;
		JAVA_LD_LIBRARY_PATH: string;
		npm_config_engine_strict: string;
		OPENCODE: string;
		LC_TIME: string;
		CFLAGS: string;
		ADDR2LINE: string;
		XDG_SEAT: string;
		CMAKE_ARGS: string;
		CPP: string;
		npm_package_version: string;
		LANGUAGE: string;
		DEBUGINFOD_URLS: string;
		OLDPWD: string;
		OBJCOPY: string;
		XAUTHORITY: string;
		npm_config_npm_version: string;
		NVM_INC: string;
		USER: string;
		OPENCODE_PID: string;
		LXQT_SESSION_CONFIG: string;
		BROWSER: string;
		PW_EXPERIMENTAL_SERVICE_WORKER_NETWORK_EVENTS: string;
		RSTUDIO_WHICH_R: string;
		npm_config_noproxy: string;
		WINDOWID: string;
		SHLVL: string;
		npm_node_execpath: string;
		_: string;
		HOME: string;
		ASDF_DIR: string;
		DESKTOP_SESSION: string;
		LC_MONETARY: string;
		npm_config_prefix: string;
		GPROF: string;
		AR: string;
		XDG_RUNTIME_DIR: string;
		XDG_SESSION_CLASS: string;
		READELF: string;
		AS: string;
		CPPFLAGS: string;
		AGENT: string;
		GXX: string;
		_CE_M: string;
		SAL_VCL_QT5_USE_CAIRO: string;
		JAVA_HOME: string;
		CPP_FOR_BUILD: string;
		GTK_OVERLAY_SCROLLING: string;
		XDG_SESSION_ID: string;
		GCC_NM: string;
		CXXFLAGS: string;
		SAL_USE_VCLPLUGIN: string;
		LOGNAME: string;
		SHELL: string;
		host_alias: string;
		PATH: string;
		npm_config_cache: string;
		TERM: string;
		NODE: string;
		CONDA_PROMPT_MODIFIER: string;
		XDG_CONFIG_HOME: string;
		SSH_AGENT_PID: string;
		npm_config_local_prefix: string;
		npm_package_name: string;
		XDG_MENU_PREFIX: string;
		XDG_CURRENT_DESKTOP: string;
		LC_MEASUREMENT: string;
		XML_CATALOG_FILES: string;
		LC_ADDRESS: string;
		npm_execpath: string;
		XDG_SESSION_PATH: string;
		npm_config_node_gyp: string;
		build_alias: string;
		CC: string;
		MAKELEVEL: string;
		DISPLAY: string;
		LANG: string;
		CONDA_PREFIX: string;
		DOTNET_BUNDLE_EXTRACT_BASE_DIR: string;
		_CONDA_ROOT: string;
		DEBUG_CFLAGS: string;
		EDITOR: string;
		LC_TELEPHONE: string;
		CONDA_PYTHON_EXE: string;
		MESON_ARGS: string;
		npm_config_ignore_scripts: string;
		XDG_SESSION_DESKTOP: string;
		STRINGS: string;
		XDG_DATA_HOME: string;
		MAKEFLAGS: string;
		npm_lifecycle_script: string;
		GSETTINGS_SCHEMA_DIR: string;
		LC_NAME: string;
		STRIP: string;
		npm_lifecycle_event: string;
		HOST: string;
		CONDA_EXE: string;
		BUILD: string;
		QT_ACCESSIBILITY: string;
		DEBUG_CPPFLAGS: string;
		CXX: string;
		LLAMA_BUILD_NUMBER: string;
		LESSCLOSE: string;
		XDG_CONFIG_DIRS: string;
		CONDA_DEFAULT_ENV: string;
		CONDA_BUILD_CROSS_COMPILATION: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * This module provides access to environment variables set _dynamically_ at runtime and that are _publicly_ accessible.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Dynamic environment variables are defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`.
 * 
 * **_Public_ access:**
 * 
 * - This module _can_ be imported into client-side code
 * - **Only** variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`) are included
 * 
 * > [!NOTE] In `dev`, `$env/dynamic` includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 * 
 * > [!NOTE] To get correct types, environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * >
 * > ```env
 * > MY_FEATURE_FLAG=
 * > ```
 * >
 * > You can override `.env` values from the command line like so:
 * >
 * > ```sh
 * > MY_FEATURE_FLAG="enabled" npm run dev
 * > ```
 * 
 * For example, given the following runtime environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://example.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.ENVIRONMENT); // => undefined, not public
 * console.log(env.PUBLIC_BASE_URL); // => "http://example.com"
 * ```
 * 
 * ```
 * 
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
