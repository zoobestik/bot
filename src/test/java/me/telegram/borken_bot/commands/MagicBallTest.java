package me.telegram.borken_bot.commands;

import me.telegram.borken_bot.lib.Utils;
import org.junit.Before;
import org.junit.Ignore;
import org.junit.Test;
import org.mockito.ArgumentCaptor;
import org.telegram.telegrambots.api.methods.send.SendMessage;
import org.telegram.telegrambots.api.objects.Chat;
import org.telegram.telegrambots.api.objects.User;
import org.telegram.telegrambots.bots.AbsSender;
import org.telegram.telegrambots.exceptions.TelegramApiException;

import static org.junit.Assert.*;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;
import static org.powermock.api.easymock.PowerMock.mockStatic;
import static org.powermock.api.easymock.PowerMock.verify;

public class MagicBallTest {
    private MagicBall ball;

    @Before
    public void setUp() {
        ball = new MagicBall();
    }

    @Test
    @Ignore
    public void testGetAnswers() throws TelegramApiException {
        mockStatic(Utils.class);

        AbsSender sender = mock(AbsSender.class);

        ArgumentCaptor<SendMessage> msgCaptor = ArgumentCaptor.forClass(SendMessage.class);
        verify(sender).sendMessage(msgCaptor.capture());

        when(Utils.getRandomInRange(any(), any())).thenReturn(5);

        ball.execute(sender, new User(), new Chat(), new String[]{"8"});

        SendMessage message = msgCaptor.getValue();

        assertEquals("Мне кажется - «да»", message.getText());
    }

    @Test
    public void testIsValidAction() {
        assertTrue(ball.isValidAction("8", null));
        assertTrue(ball.isValidAction("ball", null));
        assertTrue(ball.isValidAction("8ball", null));
        assertFalse(ball.isValidAction("_8ball", null));
    }

    @Test
    public void testGetShortNotations() {
        String[] notations = ball.getShortNotations();

        assertEquals(2, notations.length);
        assertEquals("8", notations[0]);
        assertEquals("ball", notations[1]);
    }
}