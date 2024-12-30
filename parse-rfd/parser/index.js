// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

let Asciidoctor = require('@asciidoctor/core')
let convert = require('html-to-text').convert

const asciidoc = Asciidoctor()

const parse = (content) => {
  const doc = asciidoc.load(content)

  const sections = doc
    .getSections()
    .map((section) => formatSection(section, content))
    .reduce((acc, prev) => [...acc, ...prev], [])

  const title = doc.getTitle()

  return {
    title: (title || '')
      .replace('RFD', '')
      .replace('# ', '')
      .replace('= ', '')
      .trim()
      .split(' ')
      .slice(1)
      .join(' '),
    sections,
  }
}

const formatSection = (section, content) => {
  const formattedSections = []
  for (const s of section.getSections()) {
    formattedSections.push(...formatSection(s, content))
  }
  const parentSections = []
  let level = section.getLevel() - 1
  let currSection = section.getParent()

  while (level-- && currSection) {
    if (typeof currSection.getName === 'function') {
      parentSections.push(currSection.getName())
    }
    currSection = currSection.getParent()
  }

  return [
    {
      section_id: section.getId(),
      name: section.getName(),
      content: convert(
        section
          .getBlocks()
          .filter((block) => block.context !== 'section')
          .map((block) => block.convert())
          .join(''),
      ),
      parents: parentSections,
    },
    ...formattedSections,
  ]
}

let content = require('fs').readFileSync(0, 'utf-8')
console.log(JSON.stringify(parse(content)))
