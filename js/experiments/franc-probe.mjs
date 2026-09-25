import { francAll } from 'franc-min';
const only = ['eng', 'cmn', 'hin', 'spa', 'arb', 'fra', 'ben', 'por', 'rus', 'urd'];
for (const text of ['Hawaii is a state.\n', '你好。\n', 'नमस्ते।\n', 'Hawaii es un estado.\n', 'مرحبا.\n', 'Hawaii est un etat.\n', 'নমস্কার।\n', 'Hawaii e um estado.\n', 'Гавайи это штат.\n', 'سلام۔\n', 'This sentence gives the detector enough English context.\n']) {
  console.log(JSON.stringify(text), francAll(text, { only, minLength: 1 }).slice(0, 3));
}
