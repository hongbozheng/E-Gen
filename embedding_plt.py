#!/usr/bin/env python3


import argparse
import config
import logger
import torch
import umap.umap_ as umap
import matplotlib.pyplot as plt
from sklearn.manifold import TSNE


def embedding_plt(embeddings_filepath: str, method: str, perplexity: int, seed: int) -> None:
    methods = {"UMAP", "t-SNE"}
    if method not in methods:
        logger.log_error("Invalid dimensionality reduction method!")
        logger.log_error(f"Choose from {methods}.")
        exit(1)

    embeddings = torch.load(f=embeddings_filepath)

    if method == "UMAP":
        model = umap.UMAP(n_neighbors=perplexity, n_components=2)
        mapper = model.fit(X=embeddings)
        embeds = mapper.embedding_
    elif method == "t-SNE":
        tsne = TSNE(n_components=2, perplexity=perplexity, random_state=seed)
        embeds = tsne.fit_transform(X=embeddings)

    # color_cycle = [config.colors[i // 30 % len(config.colors)] for i in range(embeds.shape[0])]
    #
    # scatter = plt.scatter(x=embeds[:, 0], y=embeds[:, 1], c=color_cycle, s=10)
    # legend_handles = [plt.Line2D(xdata=[0], ydata=[0], marker='o', color='w', label=label,
    #                              markerfacecolor=color, markersize=3, alpha=0.75) for
    #                   label, color in zip(labels, set(color_cycle))]

    scatter = plt.scatter(x=embeddings[:, 0], y=embeddings[:, 1], s=10)

    plt.rc(group="font", family="serif")
    plt.rc(group="text", usetex=True)
    # plt.legend(handles=legend_handles, loc="best", title="Class", framealpha=0.25, fontsize=6, prop={"size": 6, })
    plt.title(r"UMAP Embeddings")
    plt.xlabel(r"UMAP Dimension 1")
    plt.ylabel(r"UMAP Dimension 2")

    plt.savefig(f"{method}.png", dpi=1000)
    # plt.show()

    return


def main() -> None:
    parser = argparse.ArgumentParser(prog="embedding_plt", description="graphically illustrate embeddings")
    parser.add_argument("--embeddings_filepath", "-e", type=str, required=True, help="embeddings .pt filepath")
    parser.add_argument("--method", "-m", type=str, required=True, help="dimensionality reduction method")
    parser.add_argument("--perplexity", "-p", type=int, required=True, help="perplexity/n_neighbors")

    args = parser.parse_args()
    embeddings_filepath = args.embeddings_filepath
    method = args.method
    perplexity = args.perplexity

    logger.log_info("Start plotting embeddings...")
    embedding_plt(embeddings_filepath=embeddings_filepath, method=method, perplexity=perplexity, seed=config.SEED)
    logger.log_info("Finished plotting embeddings.")

    return


if __name__ == "__main__":
    main()