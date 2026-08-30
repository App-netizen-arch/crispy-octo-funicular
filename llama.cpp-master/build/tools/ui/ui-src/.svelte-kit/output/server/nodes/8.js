

export const index = 8;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/settings/__section__/_page.svelte.js')).default;
export const imports = [];
export const stylesheets = [];
export const fonts = [];
