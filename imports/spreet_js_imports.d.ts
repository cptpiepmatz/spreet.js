/* tslint:disable */
/* eslint-disable */
export function spreet_impl(files: SpriteSvg[], options?: Options | null): Output;
export interface Options {
    ratio?: number;
    retina?: boolean;
    unique?: boolean;
    minifyIndexFile?: boolean;
    sdf?: boolean;
    prettyJson?: boolean;
}

export interface SpriteSvg {
    name: string;
    content: Uint8Array;
}

export interface Output {
    png: Uint8Array;
    json: string;
}

export type ImplErrorKind = "USVG" | "SPREET" | "SDF_SPRITE" | "SPRITE" | "GENERATE";

export interface ImplError {
    kind: ImplErrorKind;
    msg: string;
    sprite: string | undefined;
}

