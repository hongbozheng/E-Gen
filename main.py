#!/usr/bin/env python3

import context_grammar
import logger
import argparse

eclass = []
eclass.append({'id':0,
               'node':[('Add',0,3),'x'],
               'parent':[('Mul',0,1),('Add',0,3)]})
eclass.append({'id':1,
               'node':[('Add',1,3),'y'],
               'parent':[('Mul',0,1),('Add',1,3)]})
eclass.append({'id':2,
               'node':[('Add',2,3),('Mul',0,1)],
               'parent':[('Add',2,3)]})
eclass.append({'id':3,
               'node':[0],
               'parent':[('Add',0,3),('Add',1,3),('Add',2,3),('Add',3,3),('Add',3,3)]})

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--loglevel', '-l', type=str, default='info', help='log-level', required=False)
    parser.add_argument('--csg', '-c', type=int, default=1, help='context-Sensitive grammar flag', required=False)
    args = parser.parse_args()
    csg = args.csg

    # log_level = logger.LogLevel.info

    equation = '* x y'
    logger.log_info('$$$$$$$$$$ Equation $$$$$$$$$$\n')
    logger.log_info(equation)
    logger.log_info_raw('\n\n')

    init_rw = '* e0 e1'
    ctx_g = context_grammar.ContextGrammer(eclass=eclass, init_rw=init_rw)

    logger.log_info('$$$$$$$$$$ Eclass $$$$$$$$$$\n')
    for e in ctx_g.eclass:
        logger.log_info(e)
    logger.log_info_raw('\n\n')

    ctx_g.set_grammar()
    logger.log_info('$$$$$$$$$$ Grammar $$$$$$$$$$\n')
    for g in ctx_g.grammar:
        logger.log_info(g, ctx_g.grammar[g])
    logger.log_info_raw('\n\n')

    logger.log_info('$$$$$$$$$$ Rewrite $$$$$$$$$$\n')
    if csg:
        ctx_g.csg_extract(str_=ctx_g.init_rw, idx=0)
        csg_rw = ctx_g.get_rw()
        logger.log_info_raw()
        if len(csg_rw) == len(set(csg_rw)):
            logger.log_info('RW are unique')
        else:
            logger.log_info('Duplicate RW')
            csg_rw = set(csg_rw)
            logger.log_info('TOTAL # of RW %d' % len(csg_rw))
            csg_rw = sorted(csg_rw, key=len, reverse=False)
            for rw in csg_rw:
                logger.log_info('%s' % rw)

    else:
        ctx_g.cfg_extract(str_=ctx_g.init_rw, idx=0)
        cfg_rw = ctx_g.get_rw()
        logger.log_info_raw()
        if len(cfg_rw) == len(set(cfg_rw)):
            logger.log_info('RW are unique')
            logger.log_info('TOTAL # of RW %d' % len(cfg_rw))
            cfg_rw.sort(key=len, reverse=False)
            for rw in cfg_rw:
                logger.log_info('%s' % rw)
        else:
            logger.log_info('Duplicate RW')

if __name__ == '__main__':
    main()