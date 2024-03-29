#!/usr/bin/env python3


import argparse
import config
import logger
import random


    # sin(x+a)
    # sin(ax)
    # csin(ax)
    # csin(x+a)
    # csin(x)+a
    # sin(ax+b)
    # sin(ax)+b
    # sin(x+a)+b
    # csin(ax+b)
    # csin(ax)+b
    # csin(x+b)+c
    # sin(ax+b)+c


def w_fund_exprs(fund_exprs: list[str], op_flag: bool, op: str, fund_exprs_filepath: str) -> None:
    file = open(file=fund_exprs_filepath, mode='w')

    for expr in fund_exprs:
        if not op_flag:
            file.write(f"{expr}\n")
        else:
            file.write(f"({op} {expr})\n")

    file.close()

    return


def fund_expr() -> list[str]:
    exprs = []

    rand_num = lambda: random.randint(a=2, b=9) if random.choice([True, False]) else random.randint(a=-9, b=-2)

    # x
    exprs.append("(x)")

    # x^a
    exprs.append(f"(pow x {rand_num()})")

    # x+a
    exprs.append(f"(+ x {rand_num()})")
    exprs.append(f"(- x {rand_num()})")

    # x^a+b
    exprs.append(f"(+ (pow x {rand_num()}) {rand_num()})")
    exprs.append(f"(- (pow x {rand_num()}) {rand_num()})")

    # ax
    exprs.append(f"(* {rand_num()} x)")
    exprs.append(f"(/ x {rand_num()})")

    # ax^b
    exprs.append(f"(* {rand_num()} (pow x {rand_num()}))")
    exprs.append(f"(/ (pow x {rand_num()}) {rand_num()})")

    # ax+b
    exprs.append(f"(+ (* {rand_num()} x) {rand_num()})")
    exprs.append(f"(+ (/ x {rand_num()}) {rand_num()})")
    exprs.append(f"(- (* {rand_num()} x) {rand_num()})")
    exprs.append(f"(- (/ x {rand_num()}) {rand_num()})")

    # ax^b+c
    exprs.append(f"(+ (* {rand_num()} (pow x {rand_num()})) {rand_num()})")
    exprs.append(f"(+ (/ (pow x {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(- (* {rand_num()} (pow x {rand_num()})) {rand_num()})")
    exprs.append(f"(- (/ (pow x {rand_num()}) {rand_num()}) {rand_num()})")

    # (ax+b)^c
    exprs.append(f"(pow (+ (* {rand_num()} x) {rand_num()}) {rand_num()})")
    exprs.append(f"(pow (+ (/ x {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(pow (- (* {rand_num()} x) {rand_num()}) {rand_num()})")
    exprs.append(f"(pow (- (/ x {rand_num()}) {rand_num()}) {rand_num()})")

    # a(x^b+c)
    exprs.append(f"(* {rand_num()} (+ (pow x {rand_num()}) {rand_num()}))")
    exprs.append(f"(* {rand_num()} (- (pow x {rand_num()}) {rand_num()}))")
    exprs.append(f"(/ (+ (pow x {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(/ (- (pow x {rand_num()}) {rand_num()}) {rand_num()})")

    # a(bx+c)^d
    exprs.append(f"(* {rand_num()} (pow (+ (* {rand_num()} x) {rand_num()}) {rand_num()}))")
    exprs.append(f"(* {rand_num()} (pow (+ (/ x {rand_num()}) {rand_num()}) {rand_num()}))")
    exprs.append(f"(* {rand_num()} (pow (- (* {rand_num()} x) {rand_num()}) {rand_num()}))")
    exprs.append(f"(* {rand_num()} (pow (- (/ x {rand_num()}) {rand_num()}) {rand_num()}))")
    exprs.append(f"(/ (pow (+ (* {rand_num()} x) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(/ (pow (+ (/ x {rand_num()}) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(/ (pow (- (* {rand_num()} x) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(/ (pow (- (/ x {rand_num()}) {rand_num()}) {rand_num()}) {rand_num()})")

    # (ax+b)^c+d
    exprs.append(f"(+ (pow (+ (* {rand_num()} x) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(+ (pow (+ (/ x {rand_num()}) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(+ (pow (- (* {rand_num()} x) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(+ (pow (- (/ x {rand_num()}) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(- (pow (+ (* {rand_num()} x) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(- (pow (+ (/ x {rand_num()}) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(- (pow (- (* {rand_num()} x) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(- (pow (- (/ x {rand_num()}) {rand_num()}) {rand_num()}) {rand_num()})")

    # a(x+b)^c+d
    exprs.append(f"(+ (* {rand_num()} (pow (+ x {rand_num()}) {rand_num()})) {rand_num()})")
    exprs.append(f"(+ (* {rand_num()} (pow (- x {rand_num()}) {rand_num()})) {rand_num()})")
    exprs.append(f"(+ (/ (pow (+ x {rand_num()}) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(+ (/ (pow (- x {rand_num()}) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(- (* {rand_num()} (pow (+ x {rand_num()}) {rand_num()})) {rand_num()})")
    exprs.append(f"(- (* {rand_num()} (pow (- x {rand_num()}) {rand_num()})) {rand_num()})")
    exprs.append(f"(- (/ (pow (+ x {rand_num()}) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(- (/ (pow (- x {rand_num()}) {rand_num()}) {rand_num()}) {rand_num()})")

    # a(bx)^c+d
    exprs.append(f"(+ (* {rand_num()} (pow (* {rand_num()} x) {rand_num()})) {rand_num()})")
    exprs.append(f"(+ (* {rand_num()} (pow (/ x {rand_num()}) {rand_num()})) {rand_num()})")
    exprs.append(f"(+ (/ (pow (* {rand_num()} x) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(+ (/ (pow (/ x {rand_num()}) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(- (* {rand_num()} (pow (* {rand_num()} x) {rand_num()})) {rand_num()})")
    exprs.append(f"(- (* {rand_num()} (pow (/ x {rand_num()}) {rand_num()})) {rand_num()})")
    exprs.append(f"(- (/ (pow (* {rand_num()} x) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(- (/ (pow (/ x {rand_num()}) {rand_num()}) {rand_num()}) {rand_num()})")

    # a(bx^c+d)
    exprs.append(f"(* {rand_num()} (+ (* {rand_num()} (pow x {rand_num()})) {rand_num()}))")
    exprs.append(f"(* {rand_num()} (+ (/ (pow x {rand_num()}) {rand_num()}) {rand_num()}))")
    exprs.append(f"(* {rand_num()} (- (* {rand_num()} (pow x {rand_num()})) {rand_num()}))")
    exprs.append(f"(* {rand_num()} (- (/ (pow x {rand_num()}) {rand_num()}) {rand_num()}))")
    exprs.append(f"(/ (+ (* {rand_num()} (pow x {rand_num()})) {rand_num()}) {rand_num()})")
    exprs.append(f"(/ (+ (/ (pow x {rand_num()}) {rand_num()}) {rand_num()}) {rand_num()})")
    exprs.append(f"(/ (- (* {rand_num()} (pow x {rand_num()})) {rand_num()}) {rand_num()})")
    exprs.append(f"(/ (- (/ (pow x {rand_num()}) {rand_num()}) {rand_num()}) {rand_num()})")

    # ax^b+cx
    exprs.append(f"(+ (* {rand_num()} (pow x {rand_num()})) (* {rand_num()} x))")
    exprs.append(f"(+ (* {rand_num()} (pow x {rand_num()})) (/ x {rand_num()}))")
    exprs.append(f"(+ (/ (pow x {rand_num()}) {rand_num()}) (* {rand_num()} x))")
    exprs.append(f"(+ (/ (pow x {rand_num()}) {rand_num()}) (/ x {rand_num()}))")
    exprs.append(f"(- (* {rand_num()} (pow x {rand_num()})) (* {rand_num()} x))")
    exprs.append(f"(- (* {rand_num()} (pow x {rand_num()})) (/ x {rand_num()}))")
    exprs.append(f"(- (/ (pow x {rand_num()}) {rand_num()}) (* {rand_num()} x))")
    exprs.append(f"(- (/ (pow x {rand_num()}) {rand_num()}) (/ x {rand_num()}))")

    # a(x^b+cx)
    exprs.append(f"(* (+ (pow x {rand_num()}) (* {rand_num()} x)) {rand_num()})")
    exprs.append(f"(* (+ (pow x {rand_num()}) (/ x {rand_num()})) {rand_num()})")
    exprs.append(f"(* (- (pow x {rand_num()}) (* {rand_num()} x)) {rand_num()})")
    exprs.append(f"(* (- (pow x {rand_num()}) (/ x {rand_num()})) {rand_num()})")
    exprs.append(f"(/ (+ (pow x {rand_num()}) (* {rand_num()} x)) {rand_num()})")
    exprs.append(f"(/ (+ (pow x {rand_num()}) (/ x {rand_num()})) {rand_num()})")
    exprs.append(f"(/ (- (pow x {rand_num()}) (* {rand_num()} x)) {rand_num()})")
    exprs.append(f"(/ (- (pow x {rand_num()}) (/ x {rand_num()})) {rand_num()})")

    # ax^b+cx+d
    exprs.append(f"(+ (+ (* {rand_num()} (pow x {rand_num()})) (* {rand_num()} x)) {rand_num()})")
    exprs.append(f"(+ (+ (* {rand_num()} (pow x {rand_num()})) (/ x {rand_num()})) {rand_num()})")
    exprs.append(f"(+ (+ (/ (pow x {rand_num()}) {rand_num()}) (* {rand_num()} x)) {rand_num()})")
    exprs.append(f"(+ (+ (/ (pow x {rand_num()}) {rand_num()}) (/ x {rand_num()})) {rand_num()})")
    exprs.append(f"(+ (- (* {rand_num()} (pow x {rand_num()})) (* {rand_num()} x)) {rand_num()})")
    exprs.append(f"(+ (- (* {rand_num()} (pow x {rand_num()})) (/ x {rand_num()})) {rand_num()})")
    exprs.append(f"(+ (- (/ (pow x {rand_num()}) {rand_num()}) (* {rand_num()} x)) {rand_num()})")
    exprs.append(f"(+ (- (/ (pow x {rand_num()}) {rand_num()}) (/ x {rand_num()})) {rand_num()})")
    exprs.append(f"(- (+ (* {rand_num()} (pow x {rand_num()})) (* {rand_num()} x)) {rand_num()})")
    exprs.append(f"(- (+ (* {rand_num()} (pow x {rand_num()})) (/ x {rand_num()})) {rand_num()})")
    exprs.append(f"(- (+ (/ (pow x {rand_num()}) {rand_num()}) (* {rand_num()} x)) {rand_num()})")
    exprs.append(f"(- (+ (/ (pow x {rand_num()}) {rand_num()}) (/ x {rand_num()})) {rand_num()})")
    exprs.append(f"(- (- (* {rand_num()} (pow x {rand_num()})) (* {rand_num()} x)) {rand_num()})")
    exprs.append(f"(- (- (* {rand_num()} (pow x {rand_num()})) (/ x {rand_num()})) {rand_num()})")
    exprs.append(f"(- (- (/ (pow x {rand_num()}) {rand_num()}) (* {rand_num()} x)) {rand_num()})")
    exprs.append(f"(- (- (/ (pow x {rand_num()}) {rand_num()}) (/ x {rand_num()})) {rand_num()})")

    # a(bx+c)^d+e
    exprs.append(f"(+ (* {rand_num()} (pow (+ (* {rand_num()} x) {rand_num()}))) {rand_num()})")
    exprs.append(f"(+ (* {rand_num()} (pow (+ (/ x {rand_num()}) {rand_num()}))) {rand_num()})")
    exprs.append(f"(+ (* {rand_num()} (pow (- (* {rand_num()} x) {rand_num()}))) {rand_num()})")
    exprs.append(f"(+ (* {rand_num()} (pow (- (/ x {rand_num()}) {rand_num()}))) {rand_num()})")
    exprs.append(f"(+ (/ (pow (+ (* {rand_num()} x) {rand_num()})) {rand_num()}) {rand_num()})")
    exprs.append(f"(+ (/ (pow (+ (/ x {rand_num()}) {rand_num()})) {rand_num()}) {rand_num()})")
    exprs.append(f"(+ (/ (pow (- (* {rand_num()} x) {rand_num()})) {rand_num()}) {rand_num()})")
    exprs.append(f"(+ (/ (pow (- (/ x {rand_num()}) {rand_num()})) {rand_num()}) {rand_num()})")
    exprs.append(f"(- (* {rand_num()} (pow (+ (* {rand_num()} x) {rand_num()}))) {rand_num()})")
    exprs.append(f"(- (* {rand_num()} (pow (+ (/ x {rand_num()}) {rand_num()}))) {rand_num()})")
    exprs.append(f"(- (* {rand_num()} (pow (- (* {rand_num()} x) {rand_num()}))) {rand_num()})")
    exprs.append(f"(- (* {rand_num()} (pow (- (/ x {rand_num()}) {rand_num()}))) {rand_num()})")
    exprs.append(f"(- (/ (pow (+ (* {rand_num()} x) {rand_num()})) {rand_num()}) {rand_num()})")
    exprs.append(f"(- (/ (pow (+ (/ x {rand_num()}) {rand_num()})) {rand_num()}) {rand_num()})")
    exprs.append(f"(- (/ (pow (- (* {rand_num()} x) {rand_num()})) {rand_num()}) {rand_num()})")
    exprs.append(f"(- (/ (pow (- (/ x {rand_num()}) {rand_num()})) {rand_num()}) {rand_num()})")

    return exprs


def fund_op_exprs(operators: list[str]) -> list[str]:
    exprs = []

    rand_num = lambda: random.randint(a=2, b=9) if random.choice([True, False]) else random.randint(a=-9, b=-2)

    for op in operators:
        # sin x
        exprs.append(f"({op} x)")

        # sin^2 x
        exprs.append(f"(pow ({op} x) 2)")

        # sin(x+a)
        exprs.append(f"({op} (+ x {rand_num()}))")
        exprs.append(f"({op} (- x {rand_num()}))")

        # sin^a(x+b)
        exprs.append(f"(pow ({op} (+ x {rand_num()})) {rand_num()})")
        exprs.append(f"(pow ({op} (- x {rand_num()})) {rand_num()})")

        # sin(ax)
        exprs.append(f"({op} (* {rand_num()} x))")
        exprs.append(f"({op} (/ x {rand_num()}))")

        # sin^a(bx)
        exprs.append(f"(pow ({op} (* {rand_num()} x)) {rand_num()})")
        exprs.append(f"(pow ({op} (/ x {rand_num()})) {rand_num()})")

        # csin(ax)
        exprs.append(f"(* {rand_num()} ({op} (* {rand_num()} x)))")
        exprs.append(f"(* {rand_num()} ({op} (/ x {rand_num()})))")
        exprs.append(f"(/ {rand_num()} ({op} (* {rand_num()} x)))")
        exprs.append(f"(/ {rand_num()} ({op} (/ x {rand_num()})))")

        # csin^a(bx)
        exprs.append(f"(* {rand_num()} (pow ({op} (* {rand_num()} x)) {rand_num()}))")
        exprs.append(f"(* {rand_num()} (pow ({op} (/ x {rand_num()})) {rand_num()}))")
        exprs.append(f"(/ {rand_num()} (pow ({op} (* {rand_num()} x)) {rand_num()}))")
        exprs.append(f"(/ {rand_num()} (pow ({op} (/ x {rand_num()})) {rand_num()}))")

        # csin(x+a)
        exprs.append(f"(* {rand_num()} ({op} (+ x {rand_num()})))")
        exprs.append(f"(* {rand_num()} ({op} (- x {rand_num()})))")
        exprs.append(f"(/ ({op} (+ x {rand_num()})) {rand_num()})")
        exprs.append(f"(/ ({op} (- x {rand_num()})) {rand_num()})")

        # csin^a(x+b)
        exprs.append(f"(* {rand_num()} (pow ({op} (+ x {rand_num()})) {rand_num()}))")
        exprs.append(f"(* {rand_num()} (pow ({op} (- x {rand_num()})) {rand_num()}))")
        exprs.append(f"(/ (pow ({op} (+ x {rand_num()})) {rand_num()}) {rand_num()})")
        exprs.append(f"(/ (pow ({op} (- x {rand_num()})) {rand_num()}) {rand_num()})")

        # csin(x)+a
        exprs.append(f"(+ (* {rand_num()} ({op} x)) {rand_num()})")
        exprs.append(f"(+ (/ ({op} x) {rand_num()}) {rand_num()})")
        exprs.append(f"(- (* {rand_num()} ({op} x)) {rand_num()})")
        exprs.append(f"(- (/ ({op} x) {rand_num()}) {rand_num()})")

        # csin^a(x)+b
        exprs.append(f"(+ (* {rand_num()} (pow ({op} x) {rand_num()})) {rand_num()})")
        exprs.append(f"(+ (/ (pow ({op} x) {rand_num()}) {rand_num()}) {rand_num()})")
        exprs.append(f"(- (* {rand_num()} (pow ({op} x) {rand_num()})) {rand_num()})")
        exprs.append(f"(- (/ (pow ({op} x) {rand_num()}) {rand_num()}) {rand_num()})")

        # sin(ax+b)
        exprs.append(f"({op} (+ (* {rand_num()} x) {rand_num()}))")
        exprs.append(f"({op} (+ (/ x {rand_num()}) {rand_num()}))")
        exprs.append(f"({op} (- (* {rand_num()} x) {rand_num()}))")
        exprs.append(f"({op} (- (/ x {rand_num()}) {rand_num()}))")

        # sin^a(bx+c)
        exprs.append(f"(pow ({op} (+ (* {rand_num()} x) {rand_num()})) {rand_num()})")
        exprs.append(f"(pow ({op} (+ (/ x {rand_num()}) {rand_num()})) {rand_num()})")
        exprs.append(f"(pow ({op} (- (* {rand_num()} x) {rand_num()})) {rand_num()})")
        exprs.append(f"(pow ({op} (- (/ x {rand_num()}) {rand_num()})) {rand_num()})")

        # sin(ax)+b
        exprs.append(f"(+ ({op} (* {rand_num()} x)) {rand_num()})")
        exprs.append(f"(+ ({op} (/ x {rand_num()})) {rand_num()})")
        exprs.append(f"(- ({op} (* {rand_num()} x)) {rand_num()})")
        exprs.append(f"(- ({op} (/ x {rand_num()})) {rand_num()})")

        # sin^a(bx)+c
        exprs.append(f"(+ (pow ({op} (* {rand_num()} x)) {rand_num()}) {rand_num()})")
        exprs.append(f"(+ (pow ({op} (/ x {rand_num()})) {rand_num()}) {rand_num()})")
        exprs.append(f"(- (pow ({op} (* {rand_num()} x)) {rand_num()}) {rand_num()})")
        exprs.append(f"(- (pow ({op} (/ x {rand_num()})) {rand_num()}) {rand_num()})")

        # sin(x+a)+b
        exprs.append(f"(+ ({op} (+ x {rand_num()})) {rand_num()})")
        exprs.append(f"(+ ({op} (- x {rand_num()})) {rand_num()})")
        exprs.append(f"(- ({op} (+ x {rand_num()})) {rand_num()})")
        exprs.append(f"(- ({op} (- x {rand_num()})) {rand_num()})")

        # sin^a(x+b)+c
        exprs.append(f"(+ (pow ({op} (+ x {rand_num()})) {rand_num()}) {rand_num()})")
        exprs.append(f"(+ (pow ({op} (- x {rand_num()})) {rand_num()}) {rand_num()})")
        exprs.append(f"(- (pow ({op} (+ x {rand_num()})) {rand_num()}) {rand_num()})")
        exprs.append(f"(- (pow ({op} (- x {rand_num()})) {rand_num()}) {rand_num()})")

        # csin(ax+b)
        exprs.append(f"(* {rand_num()} ({op} (+ (* {rand_num()} x) {rand_num()})))")
        exprs.append(f"(* {rand_num()} ({op} (+ (/ x {rand_num()}) {rand_num()})))")
        exprs.append(f"(* {rand_num()} ({op} (- (* {rand_num()} x) {rand_num()})))")
        exprs.append(f"(* {rand_num()} ({op} (- (/ x {rand_num()}) {rand_num()})))")
        exprs.append(f"(/ ({op} (+ (* {rand_num()} x) {rand_num()})) {rand_num()})")
        exprs.append(f"(/ ({op} (+ (/ x {rand_num()}) {rand_num()})) {rand_num()})")
        exprs.append(f"(/ ({op} (- (* {rand_num()} x) {rand_num()})) {rand_num()})")
        exprs.append(f"(/ ({op} (- (/ x {rand_num()}) {rand_num()})) {rand_num()})")

        # csin(ax)+b
        exprs.append(f"(+ (* {rand_num()} ({op} (* {rand_num()} x))) {rand_num()})")
        exprs.append(f"(+ (* {rand_num()} ({op} (/ x {rand_num()}))) {rand_num()})")
        exprs.append(f"(+ (/ ({op} (* {rand_num()} x)) {rand_num()}) {rand_num()})")
        exprs.append(f"(+ (/ ({op} (/ x {rand_num()})) {rand_num()}) {rand_num()})")
        exprs.append(f"(- (* {rand_num()} ({op} (* {rand_num()} x))) {rand_num()})")
        exprs.append(f"(- (* {rand_num()} ({op} (/ x {rand_num()}))) {rand_num()})")
        exprs.append(f"(- (/ ({op} (* {rand_num()} x)) {rand_num()}) {rand_num()})")
        exprs.append(f"(- (/ ({op} (/ x {rand_num()})) {rand_num()}) {rand_num()})")

        # csin(x+a)+b
        exprs.append(f"(+ (* {rand_num()} ({op} (+ x {rand_num()}))) {rand_num()})")
        exprs.append(f"(+ (* {rand_num()} ({op} (- x {rand_num()}))) {rand_num()})")
        exprs.append(f"(+ (/ ({op} (+ x {rand_num()})) {rand_num()}) {rand_num()})")
        exprs.append(f"(+ (/ ({op} (- x {rand_num()})) {rand_num()}) {rand_num()})")
        exprs.append(f"(- (* {rand_num()} ({op} (+ x {rand_num()}))) {rand_num()})")
        exprs.append(f"(- (* {rand_num()} ({op} (- x {rand_num()}))) {rand_num()})")
        exprs.append(f"(- (/ ({op} (+ x {rand_num()})) {rand_num()}) {rand_num()})")
        exprs.append(f"(- (/ ({op} (- x {rand_num()})) {rand_num()}) {rand_num()})")

        # sin(ax+b)+c
        exprs.append(f"(+ ({op} (+ (* {rand_num()} x) {rand_num()})) {rand_num()})")
        exprs.append(f"(+ ({op} (+ (/ x {rand_num()}) {rand_num()})) {rand_num()})")
        exprs.append(f"(+ ({op} (- (* {rand_num()} x) {rand_num()})) {rand_num()})")
        exprs.append(f"(+ ({op} (- (/ x {rand_num()}) {rand_num()})) {rand_num()})")
        exprs.append(f"(- ({op} (+ (* {rand_num()} x) {rand_num()})) {rand_num()})")
        exprs.append(f"(- ({op} (+ (/ x {rand_num()}) {rand_num()})) {rand_num()})")
        exprs.append(f"(- ({op} (- (* {rand_num()} x) {rand_num()})) {rand_num()})")
        exprs.append(f"(- ({op} (- (/ x {rand_num()}) {rand_num()})) {rand_num()})")

    return exprs


def main():
    parser = argparse.ArgumentParser(prog="fund_expr.py", description="Generate fundamental expressions")
    parser.add_argument("--seed", "-s", type=int, required=True, help="Random seed")
    parser.add_argument("--op_flag", "-f", action="store_true", default=False, required=False,
                        help="Whether to add operator at the front of the expression")
    parser.add_argument("--operator", "-o", type=str, required=False, help="operator")

    args = parser.parse_args()
    seed = args.seed
    op_flag = args.op_flag
    op = args.operator

    if op_flag and op is None:
        logger.log_error_raw("[USAGE]: fund_expr [-h] [--op_flag] --operator OPERATOR")
        logger.log_error("The following argument is  required: --operator/-o")
        exit(1)

    # general: SEED=42
    # d:       SEED=84
    random.seed(a=seed)

    logger.log_info("Start generating fundamental expressions...")
    func_exprs = []
    exprs = fund_expr()
    func_exprs.extend(exprs)
    # exprs = fund_op_exprs(operators=config.OPERATORS)
    # func_exprs.extend(exprs)
    w_fund_exprs(fund_exprs=func_exprs, op_flag=op_flag, op=op, fund_exprs_filepath=config.FUND_EXPRS_FILEPATH)
    logger.log_info(f"Finished generating fundamental expressions to '{config.FUND_EXPRS_FILEPATH}' file.")

    return


if __name__ == "__main__":
    main()