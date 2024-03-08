#!/usr/bin/env python3


import argparse
import config
import logger
import torch
import torch.nn.functional as F
import tqdm


def embedding_algebra(
        embeddings_filepath: str,
        exprs_filepath: str,
        test_filepath: str,
        embedding_algebra_filepath: str,
) -> None:
    embeddings = torch.load(f=embeddings_filepath)

    exprs_file = open(file=exprs_filepath, mode='r')
    exprs = [line.strip() for line in exprs_file.readlines()]
    exprs_file.close()

    assert embeddings.shape[0] == len(exprs) == len(set(exprs))

    test_file = open(file=test_filepath, mode='r')
    tests = [line.strip() for line in test_file.readlines()]
    test_file.close()

    res_file = open(file=embedding_algebra_filepath, mode='w')

    corrects = 0
    incorrects = 0

    progbar = tqdm.tqdm(iterable=tests)

    for line in progbar:
        expr = line.strip().split('\t')
        progbar.set_description(desc=f"[INFO]: Processing '{expr[0]}' '{expr[1]}' '{expr[2]}' '{expr[3]}'",
                                refresh=True)
        idx_0 = exprs.index(expr[0])
        idx_1 = exprs.index(expr[1])
        idx_2 = exprs.index(expr[2])
        gt_idx = exprs.index(expr[3])
        indices = [idx_0, idx_1, idx_2]

        pred_embedding = -embeddings[idx_0] + embeddings[idx_1] + embeddings[idx_2]
        cos_sim = F.cosine_similarity(x1=pred_embedding.unsqueeze(dim=0), x2=embeddings, dim=1)
        _, pred_indices = torch.topk(input=cos_sim, k=4)

        if pred_indices[0] not in indices:
            pred = pred_indices[0]
        elif pred_indices[1] not in indices:
            pred = pred_indices[1]
        elif pred_indices[2] not in indices:
            pred = pred_indices[2]
        else:
            pred = pred_indices[3]

        if pred == gt_idx:
            res_file.write(f"{expr[0]}\t{expr[1]}\t{expr[2]}\t{exprs[pred]}\n")
            corrects += 1
        else:
            res_file.write(f"{expr[0]}\t{expr[1]}\t{expr[2]}\t{exprs[pred]}\t[{exprs[gt_idx]}]\n")
            incorrects += 1

    res_file.close()

    logger.log_info(f"Accuracy: {corrects/(corrects+incorrects)*100:.4f}%")

    return


def main() -> None:
    parser = argparse.ArgumentParser(prog="embed_algebra", description="embedding algebra")
    parser.add_argument("--embeddings_filepath", "-p", type=str, required=True, help="embeddings .pt filepath")
    parser.add_argument("--exprs_filepath", "-e", type=str, required=True, help="exprs pool filepath")
    parser.add_argument("--test_filepath", "-t", type=str, required=True, help="test filepath")

    args = parser.parse_args()
    embeddings_filepath = args.embeddings_filepath
    exprs_filepath = args.exprs_filepath
    test_filepath = args.test_filepath

    logger.log_info("Start embedding algebra test...")
    embedding_algebra(embeddings_filepath=embeddings_filepath, exprs_filepath=exprs_filepath,
                      test_filepath=test_filepath, embedding_algebra_filepath=config.EMBEDDING_ALGEBRA_FILEPATH)
    logger.log_info("Finish embedding algebra test.")

    return


if __name__ == "__main__":
    main()