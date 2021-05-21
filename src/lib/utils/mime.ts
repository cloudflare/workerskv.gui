// TODO: Real mimetype sniffing
// @see https://mimesniff.spec.whatwg.org/
// const MIMES = [ ... ];

// export function sniff(buffer: Uint8Array) {
// 	let i=0, mime
// 	for (var i = 0, l = mime.mask.length; i < l; ++i) {
// 		if ((bytes[i] & mime.mask[i]) - mime.pattern[i] !== 0) {
// 				return false;
// 		}
// }
// return true;

function isTag(tag: 'video' | 'img' | 'audio', src: string): Promise<boolean> {
	return new Promise(res => {
		let elem = document.createElement(tag);
		elem.onerror = () => res(false);
		elem.onload = () => res(true);
		elem.src = src;
	});
}

export function toObjectURL(points: number[]): string {
	return URL.createObjectURL(
		new Blob([new Uint8Array(points)])
	);
}

export type Category = 'text' | 'json' | 'media/image' | 'media/video' | 'media/audio';

/** @param value The codepoints; eg "[117,12,18,...]" */
export async function parse(value: string): Promise<{ value: string, mimetype: Category }> {
	let mimetype: Category;
	let points: number[] = JSON.parse(value);
	let objurl = toObjectURL(points);

	try {
		if (await isTag('img', objurl)) mimetype = 'media/image';
		else if (await isTag('video', objurl)) mimetype = 'media/video';
		else if (await isTag('audio', objurl)) mimetype = 'media/audio';
		if (mimetype) return { value, mimetype };
	} finally {
		URL.revokeObjectURL(objurl);
	}

	let text: string = String.fromCharCode.apply(null, points);

	try {
		JSON.parse(text);
		mimetype = 'json';
	} catch (err) {
		mimetype = 'text';
	}

	return { value: text, mimetype };
}
