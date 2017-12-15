# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: hsabouri <hsabouri@student.42.fr>          +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2017/12/15 14:55:51 by hsabouri          #+#    #+#              #
#    Updated: 2017/12/15 15:16:50 by hsabouri         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME = computor-v1
RED = \x1b[31m
CYAN = \x1b[36m
END = \x1b[0m

$(NAME):
	cargo build --release
	@echo "Executable is located at \"target/release/$(CYAN)$(NAME)$(END)\""

all: $(NAME)

debug:
	@cargo build
	@echo "Executable is located at \"target/release/$(CYAN)$(NAME)$(END)\""

clean:
	@echo "$(RED)Deleted$(END) build files"

fclean: clean
	@rm -rf target
	@echo "$(RED)Deleted$(END) executable"

re: fclean $(NAME)

.PHONY: all clean fclean re
