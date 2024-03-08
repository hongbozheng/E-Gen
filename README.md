# Model and Data Analysis

## UMAP & t-SNE Analysis
##### Check command line input help
```
./embedding_plt.py -h
```

#### Graphically illustrate embeddings
```
./embedding_plt.py  -e <embeddings filepath> -m <method> -p <perplexity>
```
- `<embeddings filepath>` - embeddings `.pt` filepath containing expressions embeddings
- `<method>` - dimensionality reduction method `[UMAP, t-SNE]`
- `<perplexity>` - number of nearest neighbors

## Embedding Algebra
##### Check command line input help
```
./embedding_algebra.py -h
```

##### Compute embedding algebra results
```
./embedding_algebra.py -p <embeddings filepath> -e <exprs filepath> -t <test filepath>
```
- `<embeddings filepath>` - embeddings `.pt` filepath containing embeddings of all expressions in expressions pool
- `<exprs filepath>` - expressions pool filepath
- `<test filepath>` - embedding algebra test filepath