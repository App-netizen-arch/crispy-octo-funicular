

export const index = 4;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/(chat)/_page.svelte.js')).default;
export const universal = {
  "load": null
};
export const universal_id = "src/routes/(chat)/+page.ts";
export const imports = [];
export const stylesheets = [];
export const fonts = [];
