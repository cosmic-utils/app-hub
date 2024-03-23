import { themes } from '$lib/themes'

// @ts-ignore
export const handle = async ({ event, resolve }) => {
    const theme = event.cookies.get('theme')

    if (!theme || !themes.includes(theme)) {
        return await resolve(event)
    }

    return await resolve(event, {
        // @ts-ignore
        transformPageChunk: ({ html }) => {
            return html.replace('data-theme=""', `data-theme="${theme}"`)
        },
    })
}