import config
import logger

class ContextGrammer:
    def __init__(self, eclass: list, init_rw: str) -> None:
        self.eclass = eclass
        self.init_rw = init_rw
        self.grammar = {}
        self.rw = []
        return

    def set_grammar(self) -> None:
        self.grammar = {}
        for e in self.eclass:
            enode = []
            for node in e['node']:
                if type(node) == tuple:
                    nstr = ''
                    for i in node:
                        if type(i) == str:
                            nstr+=config.operator[i]
                        else:
                            nstr+=' e'+str(i)
                    enode.append(nstr)
                    continue
                enode.append(node)
            self.grammar['e'+str(e['id'])] = enode
        return

    def get_rw(self) -> list:
        return self.rw

    def get_grammer(self) -> dict:
        return self.grammar

    def is_operator(self, op: str) -> bool:
        if op in config.operator_list:
            return True
        return False

    def is_var(self, op: str) -> bool:
        if op in config.var_list:
            return True
        return False

    def is_const(self, op: str) -> bool:
        try:
            int(op)
            return True
        except:
            return False

    '''
    context-free grammar
    '''
    def cfg_extract(self, str_, idx) -> None:
        logger.log_debug_raw('-' * 35)
        logger.log_debug('Function Call %d' % idx)
        expr = str_.split(' ')
        logger.log_debug_raw('[EXPR]: ', expr)

        term = False

        for i in range(len(expr)):
            op = expr[i]
            if self.is_operator(op=op) or self.is_var(op=op) or self.is_const(op=op):
                continue
            logger.log_debug_raw('[ OP ]:  %s' % op)
            rw_list = self.grammar[str(expr[i])]
            for k in range(len(rw_list)):
                rw = rw_list[k]
                logger.log_debug_raw('[SSTR]:  %s' % str_)
                logger.log_debug_raw('[ RW ]:  %s' % rw)
                prev_str = str_
                str_ = str_.replace(op, str(rw), 1)
                logger.log_debug_raw('[AFTR]:  %s' % str_)

                if len(str_) >= 20:
                    logger.log_debug('STR exceeds length limit, Try another RW...')
                    str_ = prev_str
                    continue
                if 'e' not in str_ and k == len(rw_list)-1:
                    logger.log_info_raw('[FINAL]: %s' % str_)
                    self.rw.append(str_)
                    term = True
                    break
                elif 'e' not in str_:
                    logger.log_info_raw('[FINAL]: %s' % str_)
                    self.rw.append(str_)
                    str_ = prev_str
                else:
                    self.cfg_extract(str_, idx+1)
                    logger.log_debug('Back to Function Call %d' % idx)
                    str_ = prev_str
                    if k == len(rw_list)-1:
                        term = True
                        break
            if term:
                break
        logger.log_debug('Finish Function Call %d' % idx)
        logger.log_debug_raw('-' * 35)
        return

    '''
    context-sensitive grammar
    '''
    def csg_extract(self, str_, idx):
        logger.log_debug_raw('-' * 35)
        logger.log_debug('Function Call %d' % idx)
        expr = str_.split(' ')
        logger.log_debug_raw('[EXPR]: ', expr)

        term = False

        for i in range(len(expr)):
            op = expr[i]
            if self.is_operator(op=op) or self.is_var(op=op) or self.is_const(op=op):
                continue
            rw_list = self.grammar[str(expr[i])]
            for k in range(len(rw_list)):
                rw = rw_list[k]
                logger.log_debug_raw('[SSTR]:  %s' % str_)
                logger.log_debug_raw('[ RW ]:  %s' % rw)
                prev_str = str_
                str_ = str_.replace(op, str(rw), 1)
                logger.log_debug_raw('[AFTR]:  %s' % str_)

                if len(str_) >= 20:
                    logger.log_debug('STR exceeds length limit, Try another RW...')
                    str_ = prev_str
                    continue
                if 'e' not in str_ and k == len(rw_list)-1:
                    logger.log_info_raw('[FINAL]: %s' % str_)
                    self.rw.append(str_)
                    term = True
                    break
                elif 'e' not in str_:
                    logger.log_info_raw('[FINAL]: %s' % str_)
                    self.rw.append(str_)
                    str_ = prev_str
                else:
                    self.csg_extract(str_, idx+1)
                    logger.log_debug('Back to Function Call %d' % idx)
                    str_ = prev_str
            if term:
                break
        logger.log_debug('Finish Function Call %d' % idx)
        logger.log_debug_raw('-' * 35)