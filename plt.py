#!/usr/bin/env python3


import config
import logger
import torch
import umap.umap_ as umap
import matplotlib.pyplot as plt
from sklearn.manifold import TSNE


def main() -> None:
    embeds = torch.load(f="embeds.pt")
    # print(embeds)
    # print(embeds.shape)
    # model = umap.UMAP(n_neighbors=500, n_components=2)
    # mapper = model.fit(X=embeds)
    # embeds = mapper.embedding_

    tsne = TSNE(n_components=2, perplexity=500, random_state=42)
    embeds = tsne.fit_transform(X=embeds)

    labels = [
        "sin",
        "cos",
        "tan",
        "csc",
        "sec",
        "cot",
        "ln",
        "sinh",
        "cosh",
        "tanh",
        "asin",
        "acos",
        "atan",
    ]

    # color_cycle = color_cycle = [colors[i // 30 % len(colors)] for i in range(embeds.shape[0])]
    #
    # scatter = plt.scatter(x=embeds[:, 0], y=embeds[:, 1], c=color_cycle, s=10)
    # legend_handles = [plt.Line2D(xdata=[0], ydata=[0], marker='o', color='w', label=label,
    #                              markerfacecolor=color, markersize=3, alpha=0.25) for
    #                   label, color in zip(labels, set(color_cycle))]

    scatter = plt.scatter(x=embeds[:, 0], y=embeds[:, 1], s=10)

    plt.rc(group="font", family="serif")
    plt.rc(group="text", usetex=True)
    # plt.legend(handles=legend_handles, loc="best", title="Class", framealpha=0.25, fontsize=6, prop={"size": 6,})
    plt.title(r"t-SNE Embeddings")
    plt.xlabel(r"t-SNE Dimension 1")
    plt.ylabel(r"t-SNE Dimension 2")

    plt.savefig("t-SNE.png", dpi=800)
    plt.show()

    return


if __name__ == "__main__":
    main()