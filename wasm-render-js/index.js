"use strict";

function lookup_default(obj, key, def) {
    let val = obj[key];
    if (val === undefined) {
        return def;
    } else {
        return val;
    }
}

function styleSheet(config) {
    let font_family = lookup_default(config, "font_family", "monospace");
    let font_size = lookup_default(config, "font_size", "16px");
    let style_sheet = document.createElement("style");
        style_sheet.innerHTML = `
            .prototty-terminal br {
                line-height: 0px;
                margin: 0px;
                padding: 0px;
            }
            .prototty-terminal span {
                margin: 0px;
                padding: 0px;
                font-family: ${font_family};
                font-size: ${font_size};
            }

        `;
    return style_sheet;
}

export function installStyleSheet(config) {
    document.head.appendChild(styleSheet(config));
}

class CellData {
    constructor() {
        this.clear();
    }
    clear() {
        this.character = "&nbsp";
        this.bold = false;
        this.underline = false;
        this.foreground = "rgb(255,255,255)";
        this.background = "rgb(0,0,0)";
    }
}

class Node {
    constructor(element) {
        this.element = element;
    }
    set_character(character) {
        this.element.innerHTML = character;
    }
    set_bold(bold) {
        this.element.style.fontWeight = bold ? "bold" : "normal";
    }
    set_underline(underline) {
        this.element.style.textDecoration = underline ? "underline" : "none";
    }
    set_foreground(foreground) {
        this.element.style.color = foreground;
    }
    set_background(background) {
        this.element.style.backgroundColor = background;
    }
    set_data(data) {
        this.set_character(data.character);
        this.set_bold(data.bold);
        this.set_underline(data.underline);
        this.set_foreground(data.foreground);
        this.set_background(data.background);
    }
}

class CellNode {
    constructor(node) {
        this.node = node;
        this.data = new CellData();
        this.node.set_data(this.data);
    }
    element() {
        return this.node.element;
    }
    update(data) {
        if (this.data.character !== data.character) {
            this.node.set_character(data.character);
            this.data.character = data.characater;
        }
        if (this.data.bold !== data.bold) {
            this.node.set_bold(data.bold);
            this.data.bold = data.bold;
        }
        if (this.data.underline !== data.underline) {
            this.node.set_underline(data.underline);
            this.data.underline = data.underline;
        }
        if (this.data.foreground !== data.foreground) {
            this.node.set_foreground(data.foreground);
            this.data.foreground = data.foreground;
        }
        if (this.data.background !== data.background) {
            this.node.set_background(data.background);
            this.data.background = data.background;
        }
    }
}

class Cell {
    constructor() {
        this.cell_node = new CellNode(new Node(document.createElement("span")));
        this.next_data = new CellData();
        this.foreground_depth = 0;
        this.background_depth = 0;
    }
    element() {
        return this.cell_node.element();
    }
    clear() {
        this.foreground_depth = 0;
        this.background_depth = 0;
        this.next_data.clear();
    }
    set(depth, character, bold, underline, foreground, background) {
        if (background !== null) {
            if (depth >= this.background_depth) {
                this.background_depth = depth;
                this.next_data.background = background;
            }
        }
        if (character !== null) {
            if (depth >= this.foreground_depth) {
                this.foreground_depth = depth;
                this.next_data.character = character;
            }
        }
        if (bold !== null) {
            if (depth >= this.foreground_depth) {
                this.foreground_depth = depth;
                this.next_data.bold = bold;
            }
        }
        if (underline !== null) {
            if (depth >= this.foreground_depth) {
                this.foreground_depth = depth;
                this.next_data.underline = underline;
            }
        }
        if (foreground !== null) {
            if (depth >= this.foreground_depth) {
                this.foreground_depth = depth;
                this.next_data.foreground = foreground;
            }
        }
    }
    render() {
        this.cell_node.update(this.next_data);
    }
}

export class JsGrid {
    constructor(node, width, height) {
        this.width = width;
        this.height = height;
        this.node = node;
        this.node.className = "prototty-terminal";
        this.cells = [];
        for (let i = 0; i < height; i++) {
            for (let j = 0; j < width; j++) {
                let cell = new Cell();
                this.cells.push(cell);
                this.node.appendChild(cell.element());
            }
            this.node.appendChild(document.createElement("br"));
        }
    }
    js_set_cell(x, y, depth, character, bold, underline, foreground, background) {
        if (x < 0 || y < 0 || x >= this.width || y >= this.height) {
            return;
        }
        if (character === " ") {
            character = "&nbsp";
        }
        let index = y * this.width + x;
        let cell = this.cells[index];
        cell.set(depth, character, bold, underline, foreground, background);
    }
    js_clear() {
        for (let cell of this.cells) {
            cell.clear();
        }
    }
    js_render() {
        for (let cell of this.cells) {
            cell.render();
        }
    }
}