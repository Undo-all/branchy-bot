from model_def import deepmoji_emojis
from sentence_tokenizer import SentenceTokenizer
import json
import numpy as np

def top_elements(array, k):
    ind = np.argpartition(array, -k)[-k:]
    return ind[np.argsort(array[ind])][::-1]

maxlen = 50
batch_size = 32

vocabulary = json.load(open('servers/deepmoji/data/vocabulary.json', 'r'))

st = SentenceTokenizer(vocabulary, maxlen)
model = deepmoji_emojis(maxlen, 'servers/deepmoji/data/deepmoji_weights.hdf5')

#print('Ready')
while True:
    sentence = input()
    tokenized, _, _ = st.tokenize_sentences([sentence])
    prob = model.predict(tokenized)[0]
    scores = []

    """
    t_token = tokenized[0]
    t_score = [sentence]
    t_prob = prob[0]
    ind_top = top_elements(t_prob, 5)
    t_score.append(sum(t_prob[ind_top]))
    t_score.extend(ind_top)
    t_score.extend([t_prob[ind] for ind in ind_top])
    """
    ind_top = top_elements(prob, 1)[0]
    print(format(ind_top, '02'), end='')
