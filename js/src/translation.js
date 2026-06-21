import { LinkQuery } from './query.js';

export class TranslationTemplate {
  constructor(language, text) {
    this.language = language;
    this.text = text;
  }
}

export class TranslationRule {
  constructor(name, query) {
    this.name = name;
    this.query = query;
    this.templates = [];
  }

  withTemplate(language, text) {
    this.templates.push(new TranslationTemplate(language, text));
    return this;
  }

  templateFor(language) {
    return this.templates.find((template) => template.language === language);
  }
}

export class TranslationRuleSet {
  constructor(name, rules = []) {
    this.name = name;
    this.rules = rules;
  }

  withRule(rule) {
    this.rules.push(rule);
    return this;
  }

  render(targetLanguage, network) {
    for (const rule of this.rules) {
      const template = rule.templateFor(targetLanguage);
      if (template && network.find(rule.query).length > 0) {
        return template.text;
      }
    }
    return network.reconstructText();
  }

  toLino() {
    return JSON.stringify({
      name: this.name,
      rules: this.rules.map((rule) => ({
        name: rule.name,
        query: {
          linkType: rule.query.linkType,
          term: rule.query.term,
          language: rule.query.language,
          named: rule.query.named,
          sexpression: rule.query.sexpression,
        },
        templates: rule.templates,
      })),
    });
  }

  static fromLino(source) {
    const parsed = JSON.parse(source);
    return new TranslationRuleSet(
      parsed.name,
      parsed.rules.map((rule) => {
        const query = new LinkQuery(rule.query);
        const restored = new TranslationRule(rule.name, query);
        for (const template of rule.templates) {
          restored.withTemplate(template.language, template.text);
        }
        return restored;
      }),
    );
  }
}
