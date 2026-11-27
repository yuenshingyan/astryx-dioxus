// Adapted from Astryx utils/typeaheadMatch.ts. See THIRD_PARTY_NOTICES.md.
const stroked = {đ:'d', Đ:'d', ł:'l', Ł:'l', ø:'o', Ø:'o', ħ:'h', Ħ:'h', ŧ:'t', Ŧ:'t', ƀ:'b', Ƀ:'b', ı:'i'};
const foldForTypeahead = text => text.normalize('NFD').replace(/[\u0300-\u036f]/g, '')
  .replace(/[đĐłŁøØħĦŧŦƀɃı]/g, char => stroked[char]).normalize('NFC').toLowerCase();
const exactFold = text => text.normalize('NFC').replace(/I/g, 'ı').replace(/İ/g, 'i').toLowerCase();
const nextTypeaheadBuffer = (buffer, key) => {
  const folded = foldForTypeahead(key);
  if (!buffer && !folded) return null;
  return buffer && [...buffer].every(char => foldForTypeahead(char) === folded) ? key : buffer + key;
};
const findTypeaheadMatch = (items, label, query) => {
  if (/[đĐłŁøØħĦŧŦƀɃıİ\u0300-\u036f]/.test(query.normalize('NFD'))) {
    const exact = items.find(item => exactFold(label(item).trim()).startsWith(exactFold(query)));
    if (exact) return exact;
  }
  const folded = foldForTypeahead(query);
  return folded ? items.find(item => foldForTypeahead(label(item).trim()).startsWith(folded)) : undefined;
};
